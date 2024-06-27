use std::{thread, time::Duration};

use my_idle::components::*;
use my_idle::resources::*;
use my_idle::systems::*;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
pub fn main() -> anyhow::Result<()> {
    let mut world = World::new();

    //let mut mob_count = world.write_resource::<MobCount>();
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
        .with(Weapon::SWORD)
        .with(Experience::new(0))
        .with(Name {
            value: "Player".to_string(),
        })
        .with(HealthPoints {
            value: 170,
            max_value: 170,
        })
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
        .with(Experience::new(68))
        .with(Name {
            value: "Goblin".to_string(),
        })
        .with(HealthPoints {
            value: 100,
            max_value: 100,
        })
        .with(AttackPoints { value: 10 })
        .build();
    world.insert(player_entity);
    world.insert(MobCount(1));
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
        .with(InfoSystem, "before", &[])
        .with(FightSystem, "fight_system", &["before"])
        .with(DamageSystem, "damage_system", &["fight_system"])
        .with(SpawnSystem, "spawn_system", &["damage_system"])
        .with(InfoSystem, "info_system", &["spawn_system"])
        .with(HealingSystem, "healing_system", &[])
        .build();
    loop {
        println!("--------------------------------------");
        dispatcher.dispatch(&mut world);

        world.maintain();
        thread::sleep(Duration::from_secs(1))
    }
    //Ok(())
}
