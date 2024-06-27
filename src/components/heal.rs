use std::time::Duration;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::resources::Timer;

#[derive(Debug, Default, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Heal {
    pub value: usize,
    pub timer: Timer,
}
impl Heal {
    pub fn new(value: usize, speed: u64) -> Self {
        Self {
            value,
            timer: Timer::new(Duration::from_millis(speed)),
        }
    }
}
