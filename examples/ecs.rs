use std::{
    thread,
    time::{self, Duration},
};

use rand::Rng;
use specs::{
    storage::GenericReadStorage, Builder, Component, DispatcherBuilder, Entities, Entity, Join,
    NullStorage, ReadStorage, System, VecStorage, World, WorldExt, WriteStorage,
};
pub fn main() -> anyhow::Result<()> {
    let mut world = World::new();
    world.register::<BasicStats>();
    world.register::<HealthPoints>();
    world.register::<AttackPoints>();
    world.register::<Player>();
    world.register::<Mob>();
    world.register::<Live>();
    world.register::<Target>();
    world.register::<Name>();
    // Player
    let player_entity = world
        .create_entity()
        .with(Live)
        .with(Player)
        .with(BasicStats {
            strength: 40,
            constitution: 40,
            dexterity: 40,
            intelligence: 40,
            wisdom: 40,
            mental: 40,
        })
        .with(Name {
            value: "Orc Fighter".to_string(),
        })
        .with(HealthPoints { value: 170 })
        .with(AttackPoints { value: 10 })
        .build();
    // Mob
    let mob_entity = world
        .create_entity()
        .with(Live)
        .with(Mob)
        .with(BasicStats {
            strength: 40,
            constitution: 40,
            dexterity: 40,
            intelligence: 40,
            wisdom: 40,
            mental: 40,
        })
        .with(Name {
            value: "Goblin Fighter".to_string(),
        })
        .with(HealthPoints { value: 100 })
        .with(AttackPoints { value: 10 })
        .build();

    // Assign targets
    world
        .write_storage::<Target>()
        .insert(player_entity, Target { target: mob_entity })
        .unwrap();
    world
        .write_storage::<Target>()
        .insert(
            mob_entity,
            Target {
                target: player_entity,
            },
        )
        .unwrap();

    let mut dispatcher = DispatcherBuilder::new()
        //.with(InfoSystem, "info_system", &[])
        .with(FightSystem, "fight_system", &[])
        .with(InfoSystem, "info_system", &["fight_system"])
        .build();
    loop {
        dispatcher.dispatch(&mut world);

        world.maintain();
        thread::sleep(Duration::from_secs(1))
    }
    //Ok(())
}

#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct BasicStats {
    strength: usize,
    constitution: usize,
    dexterity: usize,
    intelligence: usize,
    wisdom: usize,
    mental: usize,
}
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct CombatStats {
    p_atack: usize,
    m_attack: usize,
    p_defense: usize,
    m_defence: usize,
}
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct Name {
    value: String,
}

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
struct Player;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
struct Mob;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
struct Live;
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct HealthPoints {
    value: usize,
}
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
struct AttackPoints {
    value: usize,
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Target {
    target: Entity,
}
struct InfoSystem;

impl<'a> System<'a> for InfoSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HealthPoints>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, healths, names) = data;
        for (entity, health, name) in (&entities, &healths, &names).join() {
            println!("{} HP: {}", name.value, health.value)
        }
    }
}
struct FightSystem;
impl<'a> System<'a> for FightSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BasicStats>,
        WriteStorage<'a, HealthPoints>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Mob>,
        ReadStorage<'a, Target>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let mut rng = rand::thread_rng();
        let (entities, stats, mut healths, players, mobs, targets, names) = data;
        // iterate over mobs
        for (_, char_stats, target, name) in (&mobs, &stats, &targets, &names).join() {
            let target_entity = target.target;
            if let Some(target_health) = healths.get_mut(target_entity) {
                let damage = char_stats.dexterity + rng.gen_range(1..=10);
                println!("{} produce {} damage", name.value, damage);
                target_health.value = if damage >= target_health.value {
                    0
                } else {
                    target_health.value - damage
                };

                if target_health.value == 0 {
                    let target_name = names
                        .get(target_entity)
                        .map(|x| x.value.to_owned())
                        .unwrap_or("Unknown Name".to_string());
                    println!("{} is defeated!", target_name);
                    //entities.delete(entity).expect("Failed to delete entity");
                    //break;
                }
            }
        }
        for (_, char_stats, target, name) in (&players, &stats, &targets, &names).join() {
            let target_entity = target.target;
            if let Some(target_health) = healths.get_mut(target_entity) {
                let damage = char_stats.dexterity + rng.gen_range(1..=10);
                println!("{} produce {} damage", name.value, damage);
                target_health.value = if damage >= target_health.value {
                    0
                } else {
                    target_health.value - damage
                };

                if target_health.value == 0 {
                    let target_name = names
                        .get(target_entity)
                        .map(|x| x.value.to_owned())
                        .unwrap_or("Unknown Name".to_string());
                    println!("{} is defeated!", target_name);
                    //entities.delete(entity).expect("Failed to delete entity");
                    //break;
                }
            }
        }
        for (entity, health) in (&entities, &healths).join() {
            if health.value == 0 {
                entities.delete(entity).expect("Delete entity");
            }
        }
    }
}
#[derive(Default)]
struct DeltaTime(Duration);
