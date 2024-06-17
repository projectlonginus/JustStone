use crate::structure::utils::enums::{EncryptType, HandshakeType, Packet, StoneTransferProtocol};
use crate::structure::utils::structs::define;

pub(crate) mod PacketBuilder;
pub(crate) mod StructStone;
pub(crate) mod StructStonePayload;
pub(crate) mod StructStoneHeader;
pub(crate) mod SecurePacket;
pub(crate) mod SecureHandshakePacket;
pub(crate) mod Builder;

pub fn connection() -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Connection, vec![]).packet()
}

pub fn disconnect() -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Disconnect, vec![]).packet()
}

pub fn response(msg: &str) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Response, msg).packet()
}

pub fn download(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Download, file).packet()
}

pub fn upload(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Upload, file).packet()
}

pub fn exploit(output: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::ExecuteCmd, output).packet()
}

pub fn secure_connection(handshake_type: &HandshakeType) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Connection, vec![]).handshake_packet(handshake_type).unwrap()
}

pub fn secure_disconnect() -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Disconnect, vec![]).secure_packet().unwrap()
}

pub fn secure_response(msg: &str) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Response, msg).secure_packet().unwrap()
}

pub fn secure_download(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Download, file).secure_packet().unwrap()
}

pub fn secure_upload(file: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Upload, file).secure_packet().unwrap()
}

pub fn secure_exploit(output: Vec<u8>) -> Packet {
    define::StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::ExecuteCmd, output).secure_packet().unwrap()
}
