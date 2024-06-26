use json::JsonValue;

use crate::structure::utils::{
    enums::{
        StatusCode,
        StoneTransferProtocol
    },
    structs::define::{
        EncryptionInfo,
        StructStoneHeader,
        StructStonePayload
    },
};

pub trait Builder {

}

pub trait ProtocolCodec {
    fn get_type(vec: &[u8; 4]) -> Self;
    fn to_bytes(&self) -> [u8; 4];
    fn to_string(&self) -> String;
}

pub trait TypeManager {
    fn to_json(&self) -> JsonValue;
    fn to_vec(&self) -> Vec<u8>;
}

pub trait Detector {
    fn display(&self);
    fn get_status(&self) -> StatusCode;
    fn get_type(&self) -> StoneTransferProtocol;
    fn get_size(&self) -> usize;
    fn get_encryption(&self) -> EncryptionInfo;
    fn get_header(&mut self) -> StructStoneHeader;
    fn get_payload(&mut self) -> StructStonePayload;
    fn get_sysinfo(&mut self) -> Vec<u8>;
    fn get_command(&mut self) -> Vec<u8>;
    fn get_response(&mut self) -> Vec<u8>;
    fn get_file(&mut self) -> Vec<u8>;
    fn get_stone(&mut self) -> Option<Vec<u8>>;
    fn take_header(&self) -> Option<&StructStoneHeader>;
    fn take_payload(&self) -> Option<&StructStonePayload>;
    fn take_sysinfo(&self) -> Option<&Vec<u8>>;
    fn take_command(&self) -> Option<&Vec<u8>>;
    fn take_response(&self) -> Option<&Vec<u8>>;
    fn take_file(&self) -> Option<&Vec<u8>>;
    fn take_stone(&self) -> Option<&Vec<u8>>;
    fn is_compression(&self) -> bool;
    fn is_encryption(&self) -> bool;
}