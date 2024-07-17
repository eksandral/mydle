use std::ops::{Add, AddAssign};

use serde::{Deserialize, Serialize};
use specs::{Component, VecStorage};

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Level {
    pub value: u8,
    pub experience: usize,
}
impl Level {
    pub const A: usize = 68;
    pub const BASE: usize = 3;
    pub fn new(value: u8) -> Self {
        let experience = Self::level_to_experience(value);
        Self { value, experience }
    }
    fn level_to_experience(level: u8) -> usize {
        Self::BASE.pow(level as u32) * Self::A / Self::BASE
    }
    fn experience_to_level(value: usize) -> u8 {
        if value < Self::BASE {
            return 1;
        }
        let value = (value * Self::BASE / Self::A).ilog(Self::BASE);
        if value > u8::MAX as u32 {
            return u8::MAX;
        }
        value as u8
    }
    pub fn next_level_eperience(&self) -> usize {
        Self::level_to_experience(self.value + 1)
    }
    pub fn progress(&self) -> f32 {
        let next_value = self.next_level_eperience();
        self.experience as f32 / next_value as f32
    }
}
impl Default for Level {
    fn default() -> Self {
        let value = 1;
        let experience = 0;
        Self { value, experience }
    }
}
impl Add<usize> for Level {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.experience += rhs;
        self.value = Self::experience_to_level(self.experience);
        self
    }
}
impl AddAssign<usize> for &mut Level {
    fn add_assign(&mut self, rhs: usize) {
        self.experience += rhs;
        self.value = Level::experience_to_level(self.experience);
    }
}
impl From<usize> for Level {
    fn from(experience: usize) -> Self {
        let value = Level::experience_to_level(experience);
        Self { value, experience }
    }
}
impl From<u8> for Level {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
