use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

use crate::TestData;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Equipment {
    pub helmet: Option<Helmet>,
    pub upper_body: Option<UpperBody>,
    pub lower_body: Option<LowerBody>,
    pub gloves: Option<Gloves>,
    pub boots: Option<Boots>,
}
impl TestData for Equipment {
    fn test_data() -> Self {
        Self {
            helmet: None, //Some(Helmet::test_data()),
            upper_body: Some(UpperBody::test_data()),
            lower_body: Some(LowerBody::test_data()),
            gloves: Some(Gloves::test_data()),
            boots: Some(Boots::test_data()),
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
    p_atk: usize,
    p_def: usize,
    m_atk: usize,
    m_def: usize,
    weight: usize,
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
