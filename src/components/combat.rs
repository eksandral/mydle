use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};


#[derive(Debug, Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Combat {
    pub p_attack: usize,
    pub p_defense: usize,
}
