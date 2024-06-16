use crate::structure::enums::{Packet, PacketError};
use crate::structure::structs::define::{SecureHandshakePacket, SecurePacket, StructStone};

impl TryFrom<Packet> for StructStone {
    type Error = PacketError;
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::StructStone(payload) => Ok(payload),
            _ => Err(PacketError::NotStructStone),
        }
    }
}

impl TryFrom<Packet> for SecurePacket {
    type Error = PacketError;
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::SecurePacket(payload) => Ok(payload),
            _ => Err(PacketError::NotSecurePacket),
        }
    }
}

impl TryFrom<Packet> for SecureHandshakePacket {
    type Error = PacketError;
    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::SecureHandshakePacket(payload) => Ok(payload),
            _ => Err(PacketError::NotSecureHandshakePacket),
        }
    }
}