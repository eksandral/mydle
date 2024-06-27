use std::{time::Duration, usize};

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct MobCount(pub usize);
impl MobCount {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
    pub fn dec(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DeltaTime(pub Duration);
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Timer {
    value: Duration,
    threshold: Duration,
    pub running: bool,
}

impl Timer {
    pub fn tick(&mut self, dt: Duration) -> bool {
        self.value += dt;
        if self.value > self.threshold {
            self.value -= self.threshold;
            return true;
        }
        false
    }
    pub fn start(&mut self) {
        self.running = true;
    }
    pub fn stop(&mut self) {
        self.running = false
    }
    pub fn stop_and_reset(&mut self) {
        self.stop();
        self.value = Duration::from_secs(0);
    }
    pub fn toggle(&mut self) {
        self.running = !self.running;
    }

    pub fn new(threshold: Duration) -> Self {
        Self {
            value: Default::default(),
            threshold,
            running: true,
        }
    }
    pub fn progress(&self) -> f32 {
        self.value.as_secs_f32() / self.threshold.as_secs_f32()
    }
    pub fn remains(&self) -> f32 {
        (self.threshold - self.value).as_secs_f32()
    }
}
