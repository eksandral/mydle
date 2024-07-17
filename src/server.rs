use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::network::Message as ServerMessage;
use crate::systems::*;
use eframe::wgpu::naga::MathFunction;
use specs::storage::{GenericReadStorage, GenericWriteStorage};
use specs::{AsyncDispatcher, Builder, Dispatcher, DispatcherBuilder, Entity, Join};
use specs::{World, WorldExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::data::char::{PlayerData, TargetData};
use crate::prelude::*;
pub struct Server {
    pub world: World,
}
impl Server {
    pub fn new_world() -> World {
        let mut world = World::new();
        world.register::<BasicStats>();
        world.register::<Health>();
        world.register::<Player>();
        world.register::<Mob>();
        world.register::<Target>();
        world.register::<Name>();
        world.register::<Level>();
        world.register::<SufferDamage>();
        world.register::<Weapon>();
        world.register::<Attack>();
        world.register::<Heal>();
        world.register::<LevelUp>();
        world.register::<LevelDown>();
        world.register::<Combat>();
        world.register::<Zone>();
        world.register::<MobCounter>();
        world.register::<Experience>();
        world.register::<Defeated>();
        world.register::<Offline>();
        world.register::<Armor>();

        world.insert(MobCount::default());
        world.insert(DeltaTime::default());
        world
    }
    pub fn get_dispatcher() -> Dispatcher<'static, 'static> {
        Self::get_dispatcher_builder().build()
    }
    pub fn get_async_dispatcher<R>(world: R) -> AsyncDispatcher<'static, R> {
        Self::get_dispatcher_builder().build_async(world)
    }
    pub fn get_dispatcher_builder() -> DispatcherBuilder<'static, 'static> {
        let dispatcher = DispatcherBuilder::new()
            .with(combat::FightSystem, "fight", &[])
            .with(combat::DamageSystem, "damage", &["fight"])
            .with(zone::SpawnSystem, "zone_spawn", &["damage"])
            .with(LevelSystem, "level", &["damage"])
            .with(LevelUpSystem, "recalculate_attack", &["level"])
            .with(healing::HealingSystem, "healing", &[])
            .with(
                zone::RemoveDefeated,
                "remove_defeated",
                &["zone_spawn", "damage", "level"],
            );
        dispatcher
    }
    pub fn create_player_entity(world: &mut World) -> Entity {
        let basic_stats = BasicStats::ORC;
        let player_entity = world
            .create_entity()
            .with(Player)
            .with(Weapon::sword("Default Sword".to_string(), 7))
            .with(Level::default())
            .with(Name {
                value: "Player".to_string(),
            })
            .with(Health::from_level_and_stats(1, &basic_stats))
            .with(Heal::new(10, 2000))
            .with(Attack::new(10, 2000))
            .with(basic_stats)
            .with(Combat::default())
            .with(Armor::default())
            .with(LevelUp)
            .build();
        world.insert(player_entity);
        player_entity
    }
    pub fn create_player_entity_with_id(world: &mut World, id: u32) -> Entity {
        let entity = {
            let entities = world.entities();
            entities.join().find(|x| x.id() == id)
        };
        match entity {
            Some(e) => e,
            None => Self::create_player_entity(world),
        }
    }
    pub fn prepare_player_data_from_world(
        world: &specs::World,
        entity: Entity,
    ) -> anyhow::Result<PlayerData> {
        let name_storage = world.read_storage::<Name>();
        let level_storage = world.read_storage::<Level>();
        let health_storage = world.read_storage::<Health>();
        let target_storage = world.read_storage::<Target>();
        let attack_storage = world.read_storage::<Attack>();
        let stats_storage = world.read_storage::<BasicStats>();
        let combat_storage = world.read_storage::<Combat>();
        let target = target_storage
            .get(entity)
            .map(|x| {
                name_storage
                    .get(x.target)
                    .zip(level_storage.get(x.target))
                    .zip(health_storage.get(x.target))
                    .zip(attack_storage.get(x.target))
                    .zip(stats_storage.get(x.target))
                    .zip(combat_storage.get(x.target))
                    .map(
                        |(((((name, level), health), attack), stats), combat)| TargetData {
                            id: x.target.id(),
                            name: format!("{} Lvl. {}", name.value, level.value),
                            level: level.clone(),
                            health: health.clone(),
                            attack: attack.clone(),
                            stats: stats.clone(),
                            combat: combat.clone(),
                        },
                    )
            })
            .flatten();
        let player_data = PlayerData {
            id: entity.id(),
            name: name_storage
                .get(entity)
                .map(|x| x.value.to_owned())
                .ok_or(anyhow::anyhow!("No Name component for the player"))?,
            level: level_storage
                .get(entity)
                .map(|x| x.to_owned())
                .ok_or(anyhow::anyhow!("No Level component for the player"))?,
            health: health_storage
                .get(entity)
                .map(|x| x.clone())
                .ok_or(anyhow::anyhow!("No Health component for the player"))?,
            attack: attack_storage
                .get(entity)
                .map(|x| x.clone())
                .ok_or(anyhow::anyhow!("No Health component for the player"))?,
            stats: stats_storage
                .get(entity)
                .map(|x| x.clone())
                .ok_or(anyhow::anyhow!("No BasicStats component for the player"))?,
            combat: combat_storage
                .get(entity)
                .map(|x| x.clone())
                .ok_or(anyhow::anyhow!("No Combat component for the player"))?,
            target,
        };

        Ok(player_data)
    }
    pub fn prepare_player_date(
        world: Arc<Mutex<specs::World>>,
        entity: Entity,
    ) -> anyhow::Result<PlayerData> {
        let world = world.lock().unwrap();
        Self::prepare_player_data_from_world(&world, entity)
    }
}
pub fn run_game_loop(
    sender: UnboundedSender<ServerMessage>,
    mut receiver: UnboundedReceiver<ServerMessage>,
) {
    let mut world = Server::new_world();
    let mut dispatcher = Server::get_dispatcher();
    log::info!("Starting game loop");
    let duration = Duration::from_millis(1000 / 300);
    let entity = Arc::new(Mutex::new(None)).clone();
    let mut current_time = std::time::Instant::now();
    loop {
        let _ = std::thread::sleep(duration);
        {
            let mut dt = world.write_resource::<DeltaTime>();
            (*dt).0 = current_time.elapsed();
        }
        let dt = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        match sender.send(ServerMessage::SystemTime(dt as u64)) {
            Ok(_) => {
                //log::debug!("Send delta time to UI {:?}", dt);
            }
            Err(e) => {
                log::error!("{}", e);
                break;
            }
        }
        while let Ok(message) = receiver.try_recv() {
            match message {
                ServerMessage::Connect(id) => {
                    log::info!("New Client Connected: {}", id);
                    let player_entity = Server::create_player_entity_with_id(&mut world, id);
                    log::debug!("New Player Entity {:?}", &player_entity);
                    let mut entity = entity.lock().unwrap();
                    *entity = Some(player_entity);
                }
                ServerMessage::EnterZone(zone) => {
                    log::info!("The Client Entered zone: {:?}", zone);
                    if let Some(e) = entity.lock().unwrap().as_ref() {
                        let mut zone_storage = world.write_storage::<Zone>();
                        zone_storage
                            .insert(*e, zone)
                            .expect("Enter zone is not possible for the entity");
                    } else {
                        log::warn!("There is no player entity to enter the {:?}", zone);
                    }
                }
                ServerMessage::LeaveZone => {
                    log::info!("The Client left a zone");
                    if let Some(e) = entity.lock().unwrap().as_ref() {
                        let mut zone_storage = world.write_storage::<Zone>();
                        zone_storage
                            .remove(*e)
                            .expect("Leave zone is not possible for the entity");
                        let p_target_entity = {
                            let target_storage = world.read_storage::<Target>();
                            target_storage.get(*e).map(|x| x.target)
                        };
                        let mut target_storage = world.write_storage::<Target>();
                        if let Some(te) = p_target_entity {
                            target_storage
                                .remove(te)
                                .expect("Cannot remove player's target from targets");
                        }
                        target_storage
                            .remove(*e)
                            .expect("Cannot remove player from target");
                    } else {
                    }
                }
                ServerMessage::UseWeapon {
                    left_hand: _,
                    weapon,
                } => {
                    if let Some(e) = entity.lock().unwrap().as_ref() {
                        let mut weapon_storage = world.write_storage::<Weapon>();
                        weapon_storage
                            .insert(*e, weapon)
                            .expect("Cannot wear a weapon");
                        let mut storage = world.write_storage::<LevelUp>();
                        storage.insert(*e, LevelUp).unwrap();
                    } else {
                        log::warn!("There is no player entity to enter use  {:?}", weapon);
                    }
                }

                ServerMessage::RemoveWeapon { left_hand: _ } => {
                    log::trace!("Received message to remove a weapon");
                    if let Some(e) = entity.lock().unwrap().as_ref() {
                        let mut weapon_storage = world.write_storage::<Weapon>();
                        weapon_storage.remove(*e).expect("Cannot remove a weapon");
                        let mut storage = world.write_storage::<LevelUp>();
                        storage.insert(*e, LevelUp).unwrap();
                    }
                }
                m => log::warn!("Received a message from GUI: {:?}", m),
            }
        }
        dispatcher.dispatch(&world);
        world.maintain();

        match entity.lock() {
            Ok(e) => {
                if let Some(entity) = e.as_ref() {
                    let data = Server::prepare_player_data_from_world(&world, *entity).unwrap();
                    let data = ServerMessage::PlayerData(data);
                    match sender.send(data) {
                        Ok(_) => {
                            //log::trace!("Send Player data to UI {:?}", entity);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => log::error!("Lock Error: {}", e),
        }
        current_time = std::time::Instant::now();
    }
    log::warn!("Game loop is finished");
}
//   #[derive(Debug)]
//   pub enum ServerMessage {
//       SystemTime(u64),
//       Binary(Vec<u8>),
//       Text(String),
//       ZoneChange(Zone),
//   }
