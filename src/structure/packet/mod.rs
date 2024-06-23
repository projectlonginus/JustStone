#![allow(non_snake_case)]

use crate::structure::utils::enums::{Packet, StoneTransferProtocol};
use crate::structure::utils::structs::define;
use crate::structure::utils::structs::define::EncryptionInfo;

pub(crate) mod PacketBuilder;
pub(crate) mod StructStone;
pub(crate) mod StructStonePayload;
pub(crate) mod StructStoneHeader;
pub(crate) mod SecurePacket;
pub(crate) mod SecureHandshakePacket;
pub(crate) mod Builder;

pub fn connection() -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Connection, vec![]).packet()
}

pub fn disconnect() -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Disconnect, vec![]).packet()
}

pub fn response(msg: &str) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Response, msg).packet()
}

pub fn download(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Download, file).packet()
}

pub fn upload(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::Upload, file).packet()
}

pub fn exploit(output: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::no_encryption(), StoneTransferProtocol::ExecuteCmd, output).packet()
}

pub fn secure_connection() -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Connection, vec![]).handshake_packet().unwrap()
}

pub fn secure_disconnect() -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Disconnect, vec![]).secure_packet().unwrap()
}

pub fn secure_response(msg: &str) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Response, msg).secure_packet().unwrap()
}

pub fn secure_download(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Download, file).secure_packet().unwrap()
}

pub fn secure_upload(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::Upload, file).secure_packet().unwrap()
}

pub fn secure_exploit(output: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptionInfo::default_encryption(), StoneTransferProtocol::ExecuteCmd, output).secure_packet().unwrap()
}
