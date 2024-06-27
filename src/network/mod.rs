use serde::{Deserialize, Serialize};

use crate::{data::char::PlayerData, prelude::Zone};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Connect(u32),
    Disconnect(u32),
    EnterZone(Zone),
    LeaveZone,
    PlayerData(PlayerData),
}
