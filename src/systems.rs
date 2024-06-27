pub mod combat;
pub mod healing;
pub mod zone;

use specs::{Entities, Join, LazyUpdate, ReadStorage, System, Write, WriteStorage};

use crate::prelude::*;
pub struct InfoSystem;

impl<'a> System<'a> for InfoSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Level>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, healths, names, xps) = data;
        for (_entity, health, name, xp) in (&entities, &healths, &names, &xps).join() {
            let next_level_xp = Level::BASE.pow(xp.value as u32 + 1);
            let pct = xp.experience * 100 / next_level_xp;
            log::trace!(
                "{} HP: {}: XP: {}, LVL: {}; Progress: {}%",
                name.value,
                health.value,
                xp.value,
                xp.value,
                pct
            )
        }
    }
}
pub struct LevelSystem;
impl<'a> System<'a> for LevelSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, Defeated>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, LevelUp>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, names, mut levels, mut defeated_storage, mut exps, mut levelups) = data;
        let mut players_xp = vec![];
        for (e, _, level, exp, name) in
            (&entities, &mut defeated_storage, &levels, &mut exps, &names).join()
        {
            log::debug!(
                "Processing current defeated entity({}) with name {}, Exp: {:?}",
                e.id(),
                name.value,
                exp
            );
            for (e_src, xp) in &exp.exp {
                let xp = xp * level.value as usize * 2;
                players_xp.push((*e_src, xp));
            }
            exp.clear();
        }
        for (e, xp) in &players_xp {
            if let Some(mut level) = levels.get_mut(*e) {
                let old_level = level.value;
                level += *xp as usize;
                if old_level < level.value {
                    log::debug!("LEVEL UP {:?}", names.get(*e));
                    levelups.insert(*e, LevelUp).unwrap();
                }
            }
        }
    }
}

pub struct LevelUpSystem;

impl<'a> System<'a> for LevelUpSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BasicStats>,
        ReadStorage<'a, LevelUp>,
        WriteStorage<'a, Combat>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Level>,
        ReadStorage<'a, Weapon>,
        ReadStorage<'a, Armor>,
        ReadStorage<'a, Name>,
        Write<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            stats,
            flags,
            mut combats,
            mut healths,
            levels,
            weapons,
            armors,
            names,
            update,
        ) = data;
        for (e, char_stats, _, combat, health, level, name) in (
            &entities,
            &stats,
            &flags,
            &mut combats,
            &mut healths,
            &levels,
            &names,
        )
            .join()
        {
            log::debug!(
                "Calculate new stats for {} fater leveling up: {} LVL",
                name.value,
                level.value
            );
            let level_mod = (level.value as f32 + 89.0) * 0.01; // level_mod / 100
                                                                // Calculate Attack power
            let str_mod = char_stats.str_modifier(); // str_mod / 100
            let weapon_atk = weapons.get(e).map(|x| x.p_atack as f32).unwrap_or(4.0);
            let mastery_mod = 1.085;
            let attack_power = str_mod * level_mod * weapon_atk * mastery_mod;
            combat.p_attack = attack_power as usize;
            //Calculate defence
            //
            let con_mod = char_stats.con_modifier();
            let armor_defense = armors.get(e).map(|x| x.defense() as f32).unwrap_or(4.0);

            let defense = con_mod * level_mod * armor_defense;
            combat.p_defense = defense as usize;
            log::debug!("Old Health: {:?}", health);
            health.recalculate_for_level_and_stats(level.value, &char_stats);
            log::debug!("New Health: {:?}", health);
            update.remove::<LevelUp>(e);
        }
    }
}
