pub use utils::*;

use crate::structure::{
    enums::{
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
    StructStonePayload::build(false, StoneTransferProtocol::Connection, vec![]).packet()
}

pub fn disconnect() -> Packet {
    StructStonePayload::build(false, StoneTransferProtocol::Disconnect, vec![]).packet()
}

pub fn response(msg: &str) -> Packet {
    StructStonePayload::build(false, StoneTransferProtocol::Response, msg).packet()
}

pub fn download(file: Vec<u8>) -> Packet {
    StructStonePayload::build(false, StoneTransferProtocol::Download, file).packet()
}

pub fn upload(file: Vec<u8>) -> Packet {
    StructStonePayload::build(false, StoneTransferProtocol::Upload, file).packet()
}

pub fn exploit(output: Vec<u8>) -> Packet {
    StructStonePayload::build(false, StoneTransferProtocol::ExecuteCmd, output).packet()
}
