pub use utils::*;

use crate::structure::{
    enums::{
        EncryptType,
        Packet,
        StoneTransferProtocol,
    },
    structs::define::StructStonePayload,
};

mod protocol;
mod utils;
mod editor;
mod packet;

pub fn connection() -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Connection, vec![]).packet()
}

pub fn disconnect() -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Disconnect, vec![]).packet()
}

pub fn response(msg: &str) -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Response, msg).packet()
}

pub fn download(file: Vec<u8>) -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Download, file).packet()
}

pub fn upload(file: Vec<u8>) -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::Upload, file).packet()
}

pub fn exploit(output: Vec<u8>) -> Packet {
    StructStonePayload::build(false, EncryptType::NotEncryption, StoneTransferProtocol::ExecuteCmd, output).packet()
}
