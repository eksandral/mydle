use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};
#[derive(Debug, Default, Component, Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct MobCounter {
    pub value: usize,
}
impl MobCounter {
    pub fn inc(&mut self) {
        self.value += 1;
    }

    pub fn dec(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
}
