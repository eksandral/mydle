use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::TestData;

use super::weapon::Weapon;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Equipment {
    pub helmet: Option<Helmet>,
    pub upper_body: Option<UpperBody>,
    pub lower_body: Option<LowerBody>,
    pub gloves: Option<Gloves>,
    pub boots: Option<Boots>,
    pub left_hand: Option<Weapon>,
    pub right_hand: Option<Weapon>,
}
impl TestData for Equipment {
    fn test_data() -> Self {
        Self {
            helmet: None, //Some(Helmet::test_data()),
            upper_body: Some(UpperBody::test_data()),
            lower_body: Some(LowerBody::test_data()),
            gloves: Some(Gloves::test_data()),
            boots: Some(Boots::test_data()),
            left_hand: None,
            right_hand: None,
        }
    }
}
impl Default for Equipment {
    fn default() -> Self {
        Self::test_data()
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Stats {
    pub p_atk: usize,
    pub p_def: usize,
    pub m_atk: usize,
    pub m_def: usize,
    pub weight: usize,
}
impl Stats {
    pub fn with_p_atk(mut self, value: usize) -> Self {
        self.p_atk = value;
        self
    }
    pub fn with_p_def(mut self, value: usize) -> Self {
        self.p_def = value;
        self
    }
    pub fn with_m_atk(mut self, value: usize) -> Self {
        self.m_atk = value;
        self
    }
    pub fn with_m_def(mut self, value: usize) -> Self {
        self.m_def = value;
        self
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Helmet {
    pub name: String,
    pub stats: Stats,
}
impl TestData for Helmet {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_def(4);
        Self {
            name: "Leather Helmet".to_string(),
            stats,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct UpperBody {
    pub name: String,
    pub stats: Stats,
}
impl TestData for UpperBody {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_def(2);
        Self {
            name: "Leather Chest".to_string(),
            stats,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LowerBody {
    pub name: String,
    pub stats: Stats,
}
impl TestData for LowerBody {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_def(2);
        Self {
            name: "Leather Pants".to_string(),
            stats,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Gloves {
    pub name: String,
    pub stats: Stats,
}
impl TestData for Gloves {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_def(2);
        Self {
            name: "Leather Gloves".to_string(),
            stats,
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Boots {
    pub name: String,
    pub stats: Stats,
}
impl TestData for Boots {
    fn test_data() -> Self {
        let stats = Stats::default().with_p_def(5);
        Self {
            name: "Leather Boots".to_string(),
            stats,
        }
    }
}
