use std::usize;

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use super::equipment::*;
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
