use crate::structure::enums::Packet;
use crate::structure::structs::define::{SecureHandshakePacket, SecurePacket, StructStone};

impl From<StructStone> for Packet {
    fn from(payload: StructStone) -> Self {
        Packet::StructStone(
            payload,
        )
    }
}

impl From<SecurePacket> for Packet {
    fn from(payload: SecurePacket) -> Self {
        Packet::SecurePacket(
            payload,
        )
    }
}

impl From<SecureHandshakePacket> for Packet {
    fn from(payload: SecureHandshakePacket) -> Self {
        Packet::SecureHandshakePacket(
            payload,
        )
    }
}