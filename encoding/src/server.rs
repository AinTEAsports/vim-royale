use anyhow::Result;
use deku::prelude::*;
use serde::{Serialize, Deserialize};
use std::convert::TryInto;

use crate::version::VERSION;

pub const WHO_AM_I_SERVER: u8 = 0;
pub const WHO_AM_I_CLIENT: u8 = 1;

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct PlayerStart {
    #[deku(bits = 24)]
    pub entity_id: usize,
    pub range: u16,
    pub position: (u16, u16),
    pub seed: u32,
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct PlayerPositionUpdate {
    #[deku(bits = 24)]
    pub entity_id: usize,
    pub position: (u16, u16),
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct CreateEntity {
    #[deku(bits = 24)]
    pub entity_id: usize,
    pub position: (u16, u16),
    pub info: u8,
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct DeleteEntity {
    #[deku(bits = 24)]
    pub entity_id: usize,
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct HealthUpdate {
    #[deku(bits = 24)]
    pub entity_id: usize,
    pub health: u16,
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct CirclePosition {
    pub size: u16,
    pub position: (u16, u16),
    pub seconds: u8,
}

// TODO: i think i am wrong completely on this.  probably needs to be circleupdate
// with a new position and time to move to that point
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "parent_endian", ctx = "parent_endian: deku::ctx::Endian")]
pub struct CircleStart {
    pub seconds: u8,
}

// Here's an alternative, we can include `typ` in the enum and get rid of the context passing
// if you're open to changing the format a bit
#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(
    type = "u8",
    endian = "parent_endian",
    ctx = "parent_endian: deku::ctx::Endian"
)]
pub enum Message {
    #[deku(id = "0")]
    Whoami(u8),

    #[deku(id = "1")]
    PlayerStart(PlayerStart),

    #[deku(id = "2")]
    PlayerPositionUpdate(PlayerPositionUpdate),

    #[deku(id = "3")]
    CreateEntity(CreateEntity),

    #[deku(id = "4")]
    DeleteEntity(DeleteEntity),

    #[deku(id = "5")]
    HealthUpdate(HealthUpdate),

    #[deku(id = "6")]
    CirclePosition(CirclePosition),

    // TODO: This is definitely wrong
    #[deku(id = "7")]
    CircleStart(CircleStart),

    #[deku(id = "8")]
    PlayerCount(u8),

    #[deku(id = "9")]
    PlayerQueueCount,

    #[deku(id = "10")]
    GameCount,

    #[deku(id = "11")]
    PlayerQueueCountResult(u8),

    #[deku(id = "12")]
    GameCountResult(u16),
}

#[derive(Clone, Debug, PartialEq, DekuRead, DekuWrite, Serialize, Deserialize)]
#[deku(endian = "big")]
pub struct ServerMessage {
    pub seq_nu: u16,
    pub version: u8,
    pub msg: Message, // Message here is now u8
}

impl ServerMessage {
    pub fn new(seq_nu: u16, msg: Message) -> Self {
        return Self {
            seq_nu,
            version: VERSION,
            msg,
        }
    }

    pub fn deserialize(bytes: &[u8]) -> Result<ServerMessage> {
        let (_, server_msg) = ServerMessage::from_bytes((bytes, 0))?;
        return Ok(server_msg);
    }

    pub fn serialize(self) -> Result<Vec<u8>> {
        return Ok(self.try_into()?);
    }

    pub const CLIENT_WHO_AM_I: Self = ServerMessage {
        seq_nu: 0,
        version: VERSION,
        msg: Message::Whoami(WHO_AM_I_CLIENT),
    };

    pub const SERVER_WHO_AM_I: Self = ServerMessage {
        seq_nu: 0,
        version: VERSION,
        msg: Message::Whoami(WHO_AM_I_CLIENT),
    };
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::server::PlayerStart;

    use super::{Message, ServerMessage};

    #[test]
    fn test_serialization() -> Result<()> {
        let msg = ServerMessage {
            seq_nu: 2,
            version: 3,
            msg: Message::PlayerStart(PlayerStart {
                entity_id: 5,
                position: 6,
                range: 7,
            }),
        };

        println!("codes: {:x?}", msg.serialize());

        return Ok(());
    }
}
