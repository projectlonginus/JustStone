use crate::{
    structure::{
        utils::{
            enums::{
                Packet,
                StatusCode,
                StoneTransferProtocol,
                EncryptionFlag
            },
            structs::define::{
                EncryptionInfo,
                StructStoneHeader,
                StructStonePayload
            },
            traits::define::Detector,
        }
    }
};

impl Detector for Packet {
    fn display(&self) { self.payload().unwrap().display() }
    fn get_status(&self) -> StatusCode {
        self.payload().unwrap().get_status()
    }
    fn get_type(&self) -> StoneTransferProtocol {
        self.payload().unwrap().get_type()
    }
    fn get_size(&self) -> usize {
        self.payload().unwrap().get_size()
    }
    fn get_encryption_flag(&self) -> EncryptionFlag { self.payload().unwrap().get_encryption_flag() }
    fn get_header(&mut self) -> StructStoneHeader { self.mutable_payload().unwrap().get_header() }
    fn get_payload(&mut self) -> StructStonePayload { self.mutable_payload().unwrap().get_payload() }
    fn get_sysinfo(&mut self) -> Vec<u8> {
        self.mutable_payload().unwrap().get_sysinfo()
    }
    fn get_command(&mut self) -> Vec<u8> {
        self.mutable_payload().unwrap().get_command()
    }
    fn get_response(&mut self) -> Vec<u8> {
        self.mutable_payload().unwrap().get_response()
    }
    fn get_file(&mut self) -> Vec<u8> {
        self.mutable_payload().unwrap().get_file()
    }
    fn get_stone(&mut self) -> Option<Vec<u8>> {
        self.mutable_payload().unwrap().get_stone()
    }
    fn take_header(&self) -> Option<&StructStoneHeader> {
        self.payload().unwrap().take_header()
    }
    fn take_payload(&self) -> Option<&StructStonePayload> {
        self.payload().unwrap().take_payload()
    }
    fn take_sysinfo(&self) -> Option<&Vec<u8>> {
        self.payload().unwrap().take_sysinfo()
    }
    fn take_command(&self) -> Option<&Vec<u8>> {
        self.payload().unwrap().take_command()
    }
    fn take_response(&self) -> Option<&Vec<u8>> {
        self.payload().unwrap().take_response()
    }
    fn take_file(&self) -> Option<&Vec<u8>> { self.payload().unwrap().take_file() }
    fn take_stone(&self) -> Option<&Vec<u8>> {
        self.payload().unwrap().take_stone()
    }
    fn is_compression(&self) -> bool {
        self.payload().unwrap().is_compression()
    }
    fn is_encryption(&self) -> bool {
        self.payload().unwrap().is_encryption()
    }
}