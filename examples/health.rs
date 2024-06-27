use my_idle::prelude::{Armor, BasicStats, Combat, Health, Level, Weapon};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let level = Level {
        value: 1,
        experience: 0,
    };
    let mut combat = Combat::default();
    let weapon = Weapon::SWORD;
    let armor = Armor::default();
    let stats = BasicStats::ORC;
    println!("Stats: {:#?}", stats);
    let health = Health::from_level_and_stats(level.value, &stats);
    println!("{:#?}", health);
    println!("CON mod {}", stats.con_modifier());
    println!("base hp mod {}", Health::base_from_level(level.value));
    let level_mod = (level.value as f32 + 89.0) * 0.01; // level_mod / 100
                                                        // Calculate Attack power
    let str_mod = stats.str_modifier(); // str_mod / 100
    let weapon_atk = weapon.p_atack as f32;
    let mastery_mod = 1.085;
    let attack_power = str_mod * level_mod * weapon_atk * mastery_mod;
    combat.p_attack = attack_power as usize;
    //Calculate defence
    //
    let con_mod = stats.con_modifier();
    let armor_defense = armor.defense() as f32;
    println!("a.def: {armor_defense}, con_mod {con_mod}, level_mod {level_mod} ");

    let defense = con_mod * level_mod * armor_defense;
    combat.p_defense = defense as usize;
    println!("Combat: {:#?}", combat);
    let damage_mod = 1000.0 / level.value as f32;
    let damage = attack_power / defense * damage_mod;
    println!("Damage: {damage}");
    Ok(())
}
