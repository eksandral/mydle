//use std::sync::{Arc, Mutex};

use my_idle::{network::Message as ServerMessage, server, ui};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let (gui_sender, gui_receiver) = unbounded_channel();
    let (server_sender, server_receiver) = unbounded_channel();
    let game_loop_handle = tokio::spawn(create_gameloop_task(server_sender, gui_receiver));
    create_ui_task(gui_sender, server_receiver).await;
    // Wait for both tasks to complete
    game_loop_handle.await.unwrap();
    Ok(())
}
async fn create_gameloop_task(
    sender: UnboundedSender<ServerMessage>,
    receiver: UnboundedReceiver<ServerMessage>,
) {
    server::run_game_loop(sender, receiver);
}
async fn create_ui_task(
    sender: UnboundedSender<ServerMessage>,
    receiver: UnboundedReceiver<ServerMessage>,
) {
    ui::run_ui_app(sender, receiver).await.unwrap();
}
