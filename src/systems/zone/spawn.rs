use crate::prelude::*;
use rand::Rng;
use specs::{prelude::*, storage::GenericWriteStorage, world};

pub struct SpawnSystem;

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Zone>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Target>,
        Write<'a, MobCount>,
        Write<'a, LazyUpdate>,
        Option<Read<'a, Entity>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let mut rng = rand::thread_rng();
        let (
            entities,
            players,
            zones,
            mut attacks,
            mut targets,
            mut mob_count,
            lazy_update,
            player,
        ) = data;
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
            // If there is no mobs - spawn one
            if mob_count.zone_to_mob.get(zone).is_none() {
                let monster_level = rng.gen_range(zone.mosnter_level_range());
                let mob = entities.create();
                lazy_update.insert(mob, Mob);
                lazy_update.insert(mob, Health::from_level(monster_level));
                lazy_update.insert(mob, BasicStats::GOBLIN);
                lazy_update.insert(mob, Weapon::sword("Dagger".to_string(), 6));
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
                mob_count.zone_to_mob.insert(zone.clone(), mob);
                mob_count.mob_to_zone.insert(mob, zone.clone());

                log::trace!("Spawn new mob")
            }
            if let Some(mob) = mob_count.zone_to_mob.get(zone) {
                if let Some(players_target) = targets.get_mut(*player) {
                    if &players_target.target != mob {
                        players_target.target = *mob;
                        lazy_update.insert(*mob, Target { target: *player });
                    }
                } else {
                    targets.insert(*player, Target { target: *mob }).unwrap();
                    targets.insert(*mob, Target { target: *player }).unwrap();
                }
            }

            // after spawn or when a player entered a zone and there is a mob set the target
        }
    }

    fn setup(&mut self, world: &mut specs::prelude::World) {
        <Self::SystemData as specs::shred::DynamicSystemData>::setup(&self.accessor(), world);
        world.insert(MobCount::default());
    }
}
