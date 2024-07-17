use specs::{Entities, Join, LazyUpdate, ReadStorage, System, Write};

use crate::prelude::{Defeated, Mob, MobCount, Name};

pub struct RemoveDefeated;

impl<'a> System<'a> for RemoveDefeated {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Defeated>,
        ReadStorage<'a, Mob>,
        ReadStorage<'a, Name>,
        Write<'a, LazyUpdate>,
        Write<'a, MobCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, defeated, mob_flags, names, update, mut counter) = data;
        for (e, _, _, name) in (&entities, &defeated, &mob_flags, &names).join() {
            log::debug!("Remove entity with name {}", name.value);
            update.remove::<Defeated>(e);
            counter.clean(&e);
            entities.delete(e).unwrap();
        }
    }
}
