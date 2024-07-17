use serde::{Deserialize, Serialize};

use crate::{
    data::char::PlayerData,
    prelude::{Weapon, Zone},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    SystemTime(u64),
    Connect(u32),
    Disconnect(u32),
    EnterZone(Zone),
    LeaveZone,
    PlayerData(PlayerData),
    UseWeapon { left_hand: bool, weapon: Weapon },
    RemoveWeapon { left_hand: bool},
}
