use json::JsonValue;

use crate::{
    structure::enums::StoneTransferProtocol,
    structure::structs::define::{StructStoneHeader, StructStonePayload},
};

pub trait ProtocolCodec {
    fn to_vec(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

pub trait TypeManager {
    fn to_json(&self) -> JsonValue;
    fn to_vec(&self) -> Vec<u8>;
}

pub trait Detector {
    fn display(&self);
    fn get_type(&self) -> StoneTransferProtocol;
    fn get_size(&self) -> usize;
    fn take_sysinfo(&self) -> &Vec<u8>;
    fn take_command(&self) -> &Vec<u8>;
    fn take_response(&self) -> &Vec<u8>;
    fn take_file(&self) -> &Vec<u8>;
    fn get_sysinfo(&self) -> Vec<u8>;
    fn get_command(&self) -> Vec<u8>;
    fn get_response(&self) -> Vec<u8>;
    fn get_file(&self) -> Vec<u8>;
    fn take_header(&self) -> &StructStoneHeader;
    fn take_payload(&self) -> &StructStonePayload;
    fn get_header(&self) -> StructStoneHeader;
    fn get_payload(&self) -> StructStonePayload;
    fn get_stone(&self) -> &[u8];
    fn is_compression(&self) -> bool;
}