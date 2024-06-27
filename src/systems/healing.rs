use crate::prelude::*;
use specs::prelude::*;
pub struct HealingSystem;

impl<'a> System<'a> for HealingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, BasicStats>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Heal>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, stats, mut healths, mut heals, dt) = data;
        for (_e, char_stats, health, heal) in (&entities, &stats, &mut healths, &mut heals).join() {
            if health.max_value > health.value && heal.timer.tick(dt.0) {
                health.value = health.max_value.min(
                    heal.value * health.max_value * char_stats.constitution as usize / 200
                        + health.value,
                );
            }
        }
    }
}
