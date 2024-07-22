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
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Connection, vec![]).packet()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}
impl PacketPreset for SecureSession {
    
    fn connection() -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Handshake, vec![]).handshake_packet().unwrap()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}
