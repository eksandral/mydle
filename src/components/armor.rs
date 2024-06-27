use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Armor {
    pub helmet: usize,
    pub upper_body: usize,
    pub lower_body: usize,
    pub gloves: usize,
    pub boots: usize,
}
impl Default for Armor {
    fn default() -> Self {
        Self {
            helmet: 12,     // 14
            upper_body: 16, //33
            lower_body: 10, //21
            gloves: 3,      // 9
            boots: 4,       // 8
        }
    }
}
impl Armor {
    pub fn defense(&self) -> usize {
        self.helmet + self.upper_body + self.lower_body + self.gloves + self.boots
    }
}
