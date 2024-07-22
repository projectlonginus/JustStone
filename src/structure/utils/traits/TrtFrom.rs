use crate::{
    structure::{
        utils::{
            enums::{Packet, PacketError},
            structs::define::{SecureHandshakePacket, SecurePacket, StructStone}
        }
    }
};
use crate::stprotocol::utils::{NormalSessionLayer, SecureSessionLayer};
use crate::structure::utils::enums::{SessionParsingError, Sessions};

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

impl TryFrom<Sessions> for dyn NormalSessionLayer {
    type Error = SessionParsingError;
    fn try_from(sessions: Sessions) -> Result<Self, Self::Error> {
        match sessions {
            Sessions::NormalSession(session) => Ok(session),
            _ => Err(SessionParsingError::NormalSessionParsingError("TryFrom Error".to_string())),
        }
    }
}

impl TryFrom<Sessions> for dyn SecureSessionLayer {
    type Error = SessionParsingError;
    fn try_from(sessions: Sessions) -> Result<Self, Self::Error> {
        match sessions {
            Packet::SecureHandshakePacket(session) => Ok(session),
            _ => Err(SessionParsingError::SecureSessionParsingError("TryFrom Error".to_string())),
        }
    }
}