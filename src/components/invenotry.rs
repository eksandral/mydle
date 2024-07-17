use std::usize;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use super::{equipment::*, weapon::Weapon};
use crate::TestData;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Invenotry {
    pub max_size: usize,
    pub items: Vec<Item>,
}
impl Invenotry {
    pub(crate) fn curent_size(&self) -> usize {
        self.items.len()
    }
}
impl Default for Invenotry {
    fn default() -> Self {
        Self::test_data()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Item {
    Helmet(Helmet),
    UpperBody(UpperBody),
    LowerBody(LowerBody),
    Gloves(Gloves),
    Boots(Boots),
    Shit,
    Weapon(Weapon),
}
impl TestData for Invenotry {
    fn test_data() -> Self {
        let max_size = 100;
        let items = gen_random_items(133);
        Self { max_size, items }
    }
}
fn gen_random_items(num: usize) -> Vec<Item> {
    use rand::distributions::{Distribution, Uniform};
    let items = [
        Item::Helmet(Helmet::test_data()),
        Item::UpperBody(UpperBody::test_data()),
        Item::LowerBody(LowerBody::test_data()),
        Item::Gloves(Gloves::test_data()),
        Item::Boots(Boots::test_data()),
        Item::Shit,
        Item::Weapon(Weapon::sword("Sword 1".to_string(), 7)),
        Item::Weapon(Weapon::sword("Sword 2".to_string(), 10)),
        Item::Weapon(Weapon::sword("Sword 3".to_string(), 15)),
        Item::Weapon(Weapon::sword("Sword 4".to_string(), 25)),
        Item::Weapon(Weapon::sword("Sword 5".to_string(), 35)),
        Item::Weapon(Weapon::sword("Sword 6".to_string(), 45)),
        Item::Weapon(Weapon::sword("Sword 7".to_string(), 55)),
        Item::Weapon(Weapon::sword("Sword 8".to_string(), 65)),
        Item::Weapon(Weapon::sword("Dagger 1".to_string(), 1)),
    ];
    let mut rng = rand::thread_rng();
    let idx = Uniform::from(0..items.len());
    let mut out = Vec::with_capacity(num);
    for _ in 0..num {
        let i = idx.sample(&mut rng);
        out.push(items[i].clone());
    }
    out
}
