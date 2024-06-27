use specs::{Component, Entity, VecStorage};
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct SufferDamage {
    pub damage: Vec<(usize, Entity)>,
}
impl SufferDamage {
    pub fn add_damage(&mut self, damage: usize, entity: Entity) {
        self.damage.push((damage, entity))
    }
    pub fn sum_damage(&self) -> usize {
        self.damage.iter().map(|x| x.0).sum()
    }
    pub fn clear(&mut self) {
        self.damage.clear()
    }
}
