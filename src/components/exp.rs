use std::collections::HashMap;

use specs::{Component, Entity, VecStorage};
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Experience {
    pub exp: HashMap<Entity, usize>,
}
impl Experience {
    pub fn clear(&mut self) {
        self.exp.clear()
    }
    pub fn add(&mut self, src: Entity, exp: usize) {
        self.exp
            .entry(src)
            .and_modify(|x| {
                *x += exp;
            })
            .or_insert(exp);
    }
}
