use specs::{Component, NullStorage};
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Mob;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Defeated;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Offline;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct LevelUp;
#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct LevelDown;
