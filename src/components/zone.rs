mod mob_counter;
pub use mob_counter::MobCounter;
use std::ops::Range;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Debug, Default, Component, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[storage(VecStorage)]
pub enum Zone {
    #[default]
    Zone1,
    Zone2,
    Zone3,
    Zone4,
    Zone5,
    Zone6,
    Zone7,
}
impl Zone {
    pub const VALUES: [Self; 7] = [
        Self::Zone1,
        Self::Zone2,
        Self::Zone3,
        Self::Zone4,
        Self::Zone5,
        Self::Zone6,
        Self::Zone7,
    ];
    pub fn mosnter_level_range(&self) -> Range<u8> {
        match self {
            Self::Zone1 => 1..5,
            Self::Zone2 => 5..10,
            Self::Zone3 => 10..15,
            Self::Zone4 => 15..20,
            Self::Zone5 => 20..25,
            Self::Zone6 => 25..30,
            Self::Zone7 => 30..35,
        }
    }
}
