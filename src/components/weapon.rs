use crate::{components::equipment::Stats, TestData};
use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Debug, Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Weapon {
    pub stats: Stats,
    pub name: String,
}
impl Weapon {
    pub fn sword(name: String, p_atk: usize) -> Self {
        let stats = Stats::default().with_p_atk(p_atk);
        Self { name, stats }
    }
}
impl TestData for Weapon {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_atk(6);
        Self {
            name: "A Sword".to_string(),
            stats,
        }
    }
}
