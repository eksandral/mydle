use crate::prelude::*;
use rand::Rng;
use specs::prelude::*;

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Zone>,
        WriteStorage<'a, Attack>,
        Write<'a, MobCount>,
        Write<'a, LazyUpdate>,
        Option<Read<'a, Entity>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut rng = rand::thread_rng();
        let (entities, players, zones, mut attacks, mut mob_count, lazy_update, player) = data;
        if player.is_none() {
            return;
        }
        // search for Zone entity with MobCounter property
        //for (e, zone, counter) in (&entities, &zones, &mut mob_counter).join() {
        //    // if there is not mobs schedule spawn one
        //    if counter.value == 0 {
        //        counter.inc();
        //    }
        //}
        let player = player.unwrap();
        if let Some(zone) = zones.get(*player) {
            if mob_count.0 == 0 {
                let monster_level = rng.gen_range(zone.mosnter_level_range());
                let mob = entities.create();
                lazy_update.insert(mob, Mob);
                lazy_update.insert(mob, Health::from_level(monster_level));
                lazy_update.insert(mob, BasicStats::GOBLIN);
                lazy_update.insert(mob, Weapon::DAGGER);
                lazy_update.insert(mob, Level::from(monster_level));
                lazy_update.insert(mob, Attack::new(7, 2500));
                lazy_update.insert(mob, Experience::default());
                lazy_update.insert(mob, Armor::default());
                lazy_update.insert(
                    mob,
                    Name {
                        value: "Goblin".to_string(),
                    },
                );
                lazy_update.insert(mob, Combat::default());
                lazy_update.insert(mob, LevelUp);
                for (player, _, attack) in (&entities, &players, &mut attacks).join() {
                    attack.timer.stop_and_reset();
                    lazy_update.remove::<Target>(player);
                    lazy_update.insert(mob, Target { target: player });
                    lazy_update.insert(player, Target { target: mob });
                    break;
                }
                mob_count.inc();

                log::trace!("Spawn new mob")
            }
        }
    }

    fn setup(&mut self, world: &mut specs::prelude::World) {
        <Self::SystemData as specs::shred::DynamicSystemData>::setup(&self.accessor(), world);
        world.insert(MobCount::default());
    }
}
