use std::usize;

use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

use crate::prelude::*;
pub struct DamageSystem;
impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SufferDamage>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Defeated>,
        WriteStorage<'a, Experience>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut suffer_damages,
            mut healths,
            mut d_storage,
            mut exp_storage,
            players,
            names,
        ) = data;
        for (e, damage, health, name, exp_store) in (
            &entities,
            &mut suffer_damages,
            &mut healths,
            &names,
            &mut exp_storage,
        )
            .join()
        {
            for (d, e_src) in &damage.damage {
                if players.contains(*e_src) {
                    exp_store.add(*e_src, *d);
                    //gain_xp.entry(*e_src).and_modify(|x| *x += *d).or_insert(*d);
                }
                if *d > health.value {
                    health.value = 0;
                    break;
                } else {
                    health.value -= *d;
                }
            }
            damage.clear();
            if health.value == 0 {
                log::debug!("{} is defeated", name.value);
                d_storage
                    .insert(e, Defeated)
                    .expect("Cannot flag as Defeated");
            }
        }
    }
}

pub struct FightSystem;
impl<'a> System<'a> for FightSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Level>,
        ReadStorage<'a, Target>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, SufferDamage>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Combat>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (
            entities,
            healths,
            levels,
            targets,
            names,
            mut suffer_damage,
            dt,
            mut attacks,
            combats,
        ) = data;

        for (entity, health, level, target, name, attack, combat) in (
            &entities,
            &healths,
            &levels,
            &targets,
            &names,
            &mut attacks,
            &combats,
        )
            .join()
        {
            if health.value == 0 {
                continue;
            }
            let target_entity = target.target;
            match healths.get(target_entity) {
                Some(health) if health.value > 0 => {}
                _ => continue,
            }
            let target_combat = combats.get(target_entity);
            if target_combat.is_none() {
                continue;
            }
            if !attack.timer.running {
                attack.timer.start();
            }
            if !attack.timer.tick(dt.0) {
                continue;
            }
            // Calculate Attack power
            //let str_mod = ((char_xp.value as usize - 1) * 2 + 30) * char_stats.strength as usize; // str_mod / 100
            //let level_mod = char_xp.value as usize + 89; // level_mod / 100
            //let weapon_atk = weapons.get(entity).map(|x| x.p_atack as usize).unwrap_or(4);
            //let damage = str_mod * level_mod * weapon_atk / 10_000;
            let p_atk = combat.p_attack as f32;

            // Calculate target Defense
            //let target_lvl = xps
            //    .get(target_entity)
            //    .map(|x| x.value as usize)
            //    .unwrap_or(1);
            //let target_con = stats
            //    .get(target_entity)
            //    .map(|x| x.dexterity as usize)
            //    .unwrap_or(4);
            //let target_con_mod = ((target_lvl - 1) * 2 + 30) * target_con;
            //let target_lvl_mod = target_lvl + 89;

            //let defense = target_con_mod * target_lvl_mod / 10_000;
            let p_def = target_combat.unwrap().p_defense as f32;
            let damage_mod = 1000.0 / level.value as f32;
            let final_damage = (p_atk / p_def) * damage_mod;
            let final_damage = final_damage as usize;
            let target_name = names
                .get(target_entity)
                .map(|x| x.value.to_owned())
                .unwrap_or_default();
            log::trace!(
                "{} hits {} with {}.  P.Atk {} x P.Def {}",
                name.value,
                target_name,
                final_damage,
                p_atk,
                p_def
            );
            if let Some(store) = suffer_damage.get_mut(target_entity) {
                store.add_damage(final_damage, entity);
            } else {
                let mut comp = SufferDamage::default();
                comp.add_damage(final_damage, entity);
                suffer_damage
                    .insert(target_entity, comp)
                    .expect("cannot insert damage");
            }
        }
        for (_, attack, _) in (&entities, &mut attacks, !&targets).join() {
            if attack.timer.running {
                attack.timer.stop_and_reset();
            }
        }
    }
}
