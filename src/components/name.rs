use specs::{Component, VecStorage};
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Name {
    pub value: String,
}
