mod packets;
mod protocols;
mod utils;

pub use packets::*;
pub use protocols::*;
pub use utils::*;

pub fn connection() -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::Connection, vec![]).packet()
}

pub fn disconnect() -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::Disconnect, vec![]).packet()
}

pub fn response(msg: &str) -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::Response, msg).packet()
}

pub fn download(file: Vec<u8>) -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::Download, file).packet()
}

pub fn upload(file: Vec<u8>) -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::Upload, file).packet()
}

pub fn exploit(output: Vec<u8>) -> StructStone {
    StructStonePayload::build(false, StoneTransferProtocol::ExecuteCmd, output).packet()
}