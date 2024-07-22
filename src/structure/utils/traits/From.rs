use crate::stprotocol::utils::{NormalSessionLayer, SecureSessionLayer};
use crate::structure::utils::enums::{Packet, Sessions};
use crate::structure::utils::structs::define::{SecureHandshakePacket, SecurePacket, StructStone};

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

impl From<dyn NormalSessionLayer> for Sessions {
    fn from(session: Box<dyn NormalSessionLayer>) -> Self {
        Sessions::NormalSession(
            session
        )
    }
}

impl From<dyn SecureSessionLayer> for Sessions {
    fn from(session: Box<dyn SecureSessionLayer>) -> Self {
        Sessions::SecureSession(
            session
        )
    }
}