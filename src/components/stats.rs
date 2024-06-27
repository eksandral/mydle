use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Debug, Default, Component, Clone, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct BasicStats {
    pub strength: u8,
    pub constitution: u8,
    pub dexterity: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub mental: u8,
}
impl BasicStats {
    const BASE_CON: f32 = 0.45;
    const BASE_STR: f32 = 0.29;
    pub const ORC: Self = Self {
        strength: 40,
        constitution: 47,
        dexterity: 26,
        intelligence: 18,
        wisdom: 12,
        mental: 27,
    };
    pub const GOBLIN: Self = Self {
        strength: 40,
        constitution: 45,
        dexterity: 40,
        intelligence: 30,
        wisdom: 20,
        mental: 20,
    };
    fn modifier(value: f32, base: f32) -> f32 {
        //((value - value % 10.0) * 0.001 + 0.01) * value + base
        //base * 1.0295f32.powf(value)
        base * 1.03f32.powf(value)
    }
    pub fn con_modifier(&self) -> f32 {
        let value = self.constitution as f32;
        Self::modifier(value, Self::BASE_CON)
    }
    pub fn str_modifier(&self) -> f32 {
        let value = self.strength as f32;
        Self::modifier(value, Self::BASE_STR)
    }
}
