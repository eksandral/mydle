use std::usize;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use super::prelude::BasicStats;
#[derive(Debug, Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Health {
    pub value: usize,
    pub max_value: usize,
}
impl Health {
    const BASE_HP: f32 = 67.4; //L2 = 67.4
    const A: f32 = 12.46; // L2 = 12.46
    const K: f32 = 0.14; // L2 = 0.14
    pub fn restore(&mut self) {
        self.value = self.max_value;
    }
    pub fn base_from_level(level: u8) -> f32 {
        let level = level as f32;
        let hp = Self::BASE_HP + level * Self::A + level * (level + 1.0) * Self::K / 2.0;
        hp
    }
    pub fn from_level(level: u8) -> Self {
        let hp = Self::base_from_level(level) as usize;

        Self {
            value: hp,
            max_value: hp,
        }
    }
    pub fn from_level_and_stats(level: u8, stats: &BasicStats) -> Self {
        let con_mod = stats.con_modifier();
        let base_hp = Health::base_from_level(level);
        let max_hp = con_mod * base_hp;
        Self {
            value: max_hp as usize,
            max_value: max_hp as usize,
        }
    }
    pub fn recalculate_for_level_and_stats(&mut self, level: u8, stats: &BasicStats) {
        let Self { max_value, value } = Self::from_level_and_stats(level, stats);
        self.max_value = max_value;
        self.value = value;
    }
}
