mod armor;
mod attack;
mod combat;
mod damage;
mod equipment;
mod exp;
mod flags;
mod heal;
mod health;
mod invenotry;
mod level;
mod mana;
mod name;
mod race;
mod stats;
mod target;
mod weapon;
mod zone;

pub mod prelude {
    pub use super::{
        armor::*, attack::*, combat::*, damage::*, equipment::*, exp::*, flags::*, heal::*,
        health::*, invenotry::*, level::*, mana::*, name::*, race::*, stats::*, target::*,
        weapon::*, zone::*,
    };
}
