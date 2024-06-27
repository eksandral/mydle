use specs::{Component, VecStorage};

use super::prelude::BasicStats;
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Mana {
    pub value: usize,
    pub max_value: usize,
}
impl Mana {
    const BASE_MP: usize = 40;
    pub fn restore(&mut self) {
        self.value = self.max_value;
    }
    pub fn base_from_level(level: u8) -> usize {
        5 * (level as usize - 1) + Self::BASE_MP
    }
    pub fn from_level(level: u8) -> Self {
        let hp = Self::base_from_level(level);
        Self {
            value: hp,
            max_value: hp,
        }
    }
    pub fn from_level_and_stats(level: u8, stats: &BasicStats) -> Self {
        let con_mod = (40 * (level as usize - 1) + 40) * stats.constitution as usize;
        let base_hp = Mana::base_from_level(level);
        let max_hp = con_mod * base_hp / 1000;
        Self {
            value: max_hp,
            max_value: max_hp,
        }
    }
}
