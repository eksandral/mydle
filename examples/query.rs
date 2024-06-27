use specs::{Builder, DispatcherBuilder, Entities, ReadStorage, System, WorldExt};

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let mut world = specs::World::new();
    world.register::<comps::Name>();
    world.register::<comps::Level>();
    world.register::<comps::Power>();
    {
        let _ntt1 = world
            .create_entity()
            .with(comps::Level { value: 1 })
            .with(comps::Power { value: 2 })
            .with(comps::Name { value: "Goblin" });
    }
    {
        let _ntt2 = world
            .create_entity()
            .with(comps::Level { value: 1 })
            .with(comps::Name { value: "Boblin" });
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::MySystem, "system", &[])
        .build();
    dispatcher.dispatch(&world);
    world.maintain();
    Ok(())
}

mod systems {
    use super::comps;
    use specs::{Entities, Join, ReadStorage, System};
    pub struct MySystem;
    impl<'a> System<'a> for MySystem {
        type SystemData = (
            Entities<'a>,
            ReadStorage<'a, comps::Level>,
            ReadStorage<'a, comps::Power>,
            ReadStorage<'a, comps::Name>,
        );

        fn run(&mut self, data: Self::SystemData) {
            let (entitites, levels, powers, names) = data;
            for(e, level, power, name) in (&entitites, &levels, &powers, &names).join(){
                println!("{:?}, {:?} {:?} {:?}", e, level, power, name);
            }
            for(e, level, power, name) in (&entitites, &levels, !&powers, &names).join(){
                println!("{:?}, {:?} {:?} {:?}", e, level, power, name);
            }
        }
    }
}
mod comps {

    use specs::{Component, VecStorage};
    #[derive(Debug, Component, Clone)]
    #[storage(VecStorage)]
    pub struct Level {
        pub value: usize,
    }
    #[derive(Debug, Component, Clone)]
    #[storage(VecStorage)]
    pub struct Power {
        pub value: usize,
    }
    #[derive(Debug, Component, Clone)]
    #[storage(VecStorage)]
    pub struct Name {
        pub value: &'static str,
    }
}
