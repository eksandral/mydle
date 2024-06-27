pub mod components;
pub mod data;
pub mod model;
pub mod network;
pub mod resources;
pub mod server;
pub mod systems;
pub mod ui;

pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::resources::*;
}

pub trait TestData {
    fn test_data() -> Self;
}
