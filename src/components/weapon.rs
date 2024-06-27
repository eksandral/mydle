use specs::{Component, VecStorage};
#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Weapon {
    pub p_atack: u8,
}
impl Weapon {
    pub const SWORD: Self = Self { p_atack: 7 };
    pub const DAGGER: Self = Self { p_atack: 5 };
}
