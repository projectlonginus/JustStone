#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{
    stprotocol::utils::{SecureSession, NormalSession},
    structure::{
        utils::{
            structs::{
                define::{StructStonePayload, EncryptionInfo},
            },
            enums::{Packet, StoneTransferProtocol},
        },
        utils::traits::PacketPreset
    }
};
use crate::structure::utils::enums::EncryptionFlag;

pub(crate) mod PacketBuilder;
pub(crate) mod Header;
pub(crate) mod Payload;
pub(crate) mod Handshake;
pub(crate) mod Builder;
mod test;

// fn handshake() -> Packet {
//     StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Handshake, vec![]).packet()
// } todo!("핸드셰이크 아직 안만듦;");

impl PacketPreset for NormalSession {
    fn connection() -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Connection, vec![]).packet()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}
impl PacketPreset for SecureSession {
    
    fn connection() -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Handshake, vec![]).handshake_packet().unwrap()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}
