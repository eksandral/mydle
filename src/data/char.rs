use std::usize;

use serde::{Deserialize, Serialize};

use crate::{
    prelude::{Attack, BasicStats, Combat, Health, Level},
    TestData,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    pub id: u32,
    pub name: String,
    pub level: Level,
    pub health: Health,
    pub attack: Attack,
    pub stats: BasicStats,
    pub combat: Combat,
    pub target: Option<TargetData>,
}
impl TestData for PlayerData {
    fn test_data() -> Self {
        Self {
            id: 0,
            name: "Test Player".to_string(),
            level: Level::from(100usize),
            health: Health {
                value: 78,
                max_value: 120,
            },
            attack: Attack::new(10, 10000),
            stats: BasicStats::ORC,
            target: Some(TargetData::test_data()),
            combat: Combat::default(),
        }
    }
}
impl Default for PlayerData {
    fn default() -> Self {
        Self::test_data()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TargetData {
    pub id: u32,
    pub name: String,
    pub level: Level,
    pub health: Health,
    pub attack: Attack,
    pub stats: BasicStats,
    pub combat: Combat,
}
impl TestData for TargetData {
    fn test_data() -> Self {
        Self {
            id: 1,
            name: "Goblin Boblin".to_string(),
            level: Level::new(2),
            health: Health {
                value: 50,
                max_value: 100,
            },
            attack: Attack::new(10, 1000),
            stats: BasicStats::GOBLIN,
            combat: Combat::default(),
        }
    }
}
pub trait CharData {
    fn id(&self) -> u32;
    fn name(&self) -> String;
    fn level(&self) -> Level;
    fn health(&self) -> Health;
    fn attack(&self) -> Attack;
    fn stats(&self) -> BasicStats;
    fn combat(&self) -> Combat;
}
impl CharData for &PlayerData {
    fn id(&self) -> u32 {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn level(&self) -> Level {
        self.level.clone()
    }

    fn health(&self) -> Health {
        self.health.clone()
    }
    fn attack(&self) -> Attack {
        self.attack.clone()
    }
    fn stats(&self) -> BasicStats {
        self.stats.clone()
    }
    fn combat(&self) -> Combat {
        self.combat.clone()
    }
}
impl CharData for &TargetData {
    fn id(&self) -> u32 {
        self.id
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn level(&self) -> Level {
        self.level.clone()
    }

    fn health(&self) -> Health {
        self.health.clone()
    }
    fn attack(&self) -> Attack {
        self.attack.clone()
    }
    fn stats(&self) -> BasicStats {
        self.stats.clone()
    }
    fn combat(&self) -> Combat {
        self.combat.clone()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loot {
    pub items: Vec<InventoryItem>,
    pub max_weight: usize,
}
impl Loot {
    pub fn total_weight(&self) -> usize {
        self.items.iter().fold(0, |out, cur| out + cur.weight)
    }
    pub fn test_data() -> Self {
        let items = vec![
            ("Item 1", 2).into(),
            ("Item 2", 4).into(),
            ("Item 3", 10).into(),
        ];
        Self {
            items,
            max_weight: 100,
        }
    }
}
impl Default for Loot {
    fn default() -> Self {
        Self::test_data()
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub name: String,
    pub weight: usize,
}
impl InventoryItem {
    pub fn new(name: String, weight: usize) -> Self {
        Self { name, weight }
    }
}
impl<T> From<(T, usize)> for InventoryItem
where
    T: ToString,
{
    fn from(value: (T, usize)) -> Self {
        InventoryItem::new(value.0.to_string(), value.1)
    }
}
