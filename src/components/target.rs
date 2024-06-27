use specs::{Component, Entity, VecStorage};
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Target {
    pub target: Entity,
}
