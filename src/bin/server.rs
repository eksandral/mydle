use futures::{SinkExt, StreamExt};
use my_idle::network;
use my_idle::prelude::Zone;
use my_idle::server::Server;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::time;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;

use my_idle::resources::*;
use specs::prelude::*;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let world = Arc::new(Mutex::new(Server::new_world()));
    let websocket_task = create_websocket(world.clone());
    let gameloop_task = create_gameloop(world.clone());
    let (ws, gl) = tokio::join!(websocket_task, gameloop_task);
    let _ = ws?;
    let _ = gl?;
    Ok(())
}

async fn create_gameloop(world: Arc<Mutex<World>>) -> anyhow::Result<()> {
    let mut dispatcher = Server::get_dispatcher();
    let mut interval = time::interval(Duration::from_millis(100 / 3));
    let mut last_time = Instant::now();
    loop {
        interval.tick().await;
        let mut world = world.lock().unwrap();
        {
            let mut dt = world.write_resource::<DeltaTime>();
            (*dt).0 = last_time.elapsed();
        }
        dispatcher.dispatch(&world);
        world.maintain();
        last_time = Instant::now();
        //println!("Game loop tick");
    }
}
async fn create_websocket(world: Arc<Mutex<World>>) -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9031").await.unwrap();
    println!("WebSocket server listening on 127.0.0.1:9031");

    while let Ok((stream, _)) = listener.accept().await {
        let world = world.clone();
        let _ = tokio::spawn(handle_connection(stream, world));
    }
    Ok(())
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    world: Arc<Mutex<World>>,
) -> anyhow::Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let ws_stream = accept_async(stream).await?;
    let atomic_server_time = Arc::new(AtomicU64::new(0));

    let (mut ws_write, mut ws_read) = ws_stream.split();
    log::info!("New WebSocket connection");
    let entity = Arc::new(Mutex::new(None)).clone();

    // Main endpoint to send messages to the client
    tokio::spawn(async move {
        while let Some(ref data) = rx.recv().await {
            //let data = rmp_serde::to_vec(&data).unwrap();
            ws_write.send(data.clone()).await.unwrap();
        }
    });
    // handle incomming messages
    let world_clone = world.clone();
    let atomic_server_time_clone = atomic_server_time.clone();
    let entity_clone = entity.clone();
    tokio::spawn(async move {
        while let Some(msg) = ws_read.next().await {
            match msg {
                Ok(Message::Binary(buf)) => {
                    let data: network::Message = rmp_serde::from_slice(&buf).unwrap();
                    match data {
                        network::Message::Connect(id) => {
                            log::info!("New Client Connected: {}", id);
                            let mut world = world_clone.lock().unwrap();
                            let player_entity =
                                Server::create_player_entity_with_id(&mut world, id);
                            log::debug!("New Player Entity {:?}", &player_entity);
                            let mut entity = entity_clone.lock().unwrap();
                            *entity = Some(player_entity);
                        }
                        network::Message::Disconnect(id) => {
                            log::info!("The Client Disconected: {}", id);
                        }
                        network::Message::EnterZone(zone) => {
                            log::info!("The Client Entered zone: {:?}", zone);
                            if let Some(e) = entity_clone.lock().unwrap().as_ref() {
                                let world = world_clone.lock().unwrap();
                                let mut zone_storage = world.write_storage::<Zone>();
                                zone_storage
                                    .insert(*e, zone)
                                    .expect("Enter zone is not possible for the entity");
                            }
                        }
                        network::Message::LeaveZone => {
                            if let Some(e) = entity_clone.lock().unwrap().as_ref() {
                                let world = world_clone.lock().unwrap();
                                let mut zone_storage = world.write_storage::<Zone>();
                                zone_storage
                                    .remove(*e)
                                    .expect("Leave zone is not possible for the entity");
                            }
                        }
                        _ => log::error!("Unsupported Data"),
                    }
                }
                Ok(Message::Pong(t)) => {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(&t[..]);
                    let server_time = u64::from_be_bytes(b);
                    let diff = atomic_server_time_clone.load(Ordering::Relaxed) - server_time;
                    if diff > 10 {
                        log::debug!("Client is out of time: {}", diff);
                        return;
                    }
                }

                Ok(Message::Close(m)) => {
                    if let Ok(mut entity) = entity_clone.lock() {
                        if let Some(entity) = entity.as_ref() {
                            log::debug!("Close: {:?}", m);
                            let world = world_clone.lock().unwrap();

                            let entitites = world.entities();
                            if let Ok(_) = entitites.delete(*entity) {
                                log::debug!("Need to maintain the world");
                            }
                        }
                        *entity = None;
                    }
                }
                _ => {}
            }
        }
    });
    // PING loop
    //let tx_clone = tx.clone();
    //tokio::spawn(async move {
    //    let mut interval = tokio::time::interval(Duration::from_millis(4));
    //    loop {
    //        interval.tick().await;

    //        let server_time = SystemTime::now()
    //            .duration_since(UNIX_EPOCH)
    //            .unwrap()
    //            .as_secs();
    //        atomic_server_time.store(server_time, std::sync::atomic::Ordering::Relaxed);
    //        tx_clone
    //            .send(Message::Ping(server_time.to_be_bytes().to_vec()))
    //            .unwrap();
    //    }
    //});
    // main game state update for a Player
    let mut interval = tokio::time::interval(Duration::from_millis(100 / 3));
    loop {
        interval.tick().await;
        match entity.lock() {
            Ok(e) => {
                if let Some(entity) = e.as_ref() {
                    let world = world.clone();
                    let data = Server::prepare_player_date(world, *entity)?;
                    let data = network::Message::PlayerData(data);
                    let msg = rmp_serde::to_vec(&data).unwrap();
                    let data = Message::Binary(msg);
                    if let Err(e) = tx.send(data) {
                        log::error!("Error: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => log::error!("Lock Error: {}", e),
        }
    }
    Ok(())
}
