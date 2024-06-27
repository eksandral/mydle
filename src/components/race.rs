use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Debug, Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Race {
    base_hp: usize,
    base_defense: usize,
}
impl Race {
    pub const ORC: Self = Self {
        base_hp: 80,
        base_defense: 100,
    };
    pub const HUMAN: Self = Self {
        base_hp: 80,
        base_defense: 100,
    };
}
