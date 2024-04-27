use std::fmt::Debug;

use crate::structure::{
    enums::StoneTransferProtocol,
    structs::define::{
        SecureHandshakePacket,
        StructStoneHeader,
        StructStonePayload,
    },
    traits::define::Detector,
};

impl Detector for SecureHandshakePacket {
    fn display(&self) {
        todo!()
    }

    fn get_type(&self) -> StoneTransferProtocol {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }

    fn take_sysinfo(&self) -> &Vec<u8> {
        todo!()
    }

    fn take_command(&self) -> &Vec<u8> {
        todo!()
    }

    fn take_response(&self) -> &Vec<u8> {
        todo!()
    }

    fn take_file(&self) -> &Vec<u8> {
        todo!()
    }

    fn get_sysinfo(&self) -> Vec<u8> {
        todo!()
    }

    fn get_command(&self) -> Vec<u8> {
        todo!()
    }

    fn get_response(&self) -> Vec<u8> {
        todo!()
    }

    fn get_file(&self) -> Vec<u8> {
        todo!()
    }

    fn take_header(&self) -> &StructStoneHeader {
        todo!()
    }

    fn take_payload(&self) -> &StructStonePayload {
        todo!()
    }

    fn get_header(&self) -> StructStoneHeader {
        todo!()
    }

    fn get_payload(&self) -> StructStonePayload {
        todo!()
    }

    fn get_stone(&self) -> &[u8] {
        todo!()
    }

    fn take_stone(&self) -> &[u8] {
        todo!()
    }

    fn is_compression(&self) -> bool {
        todo!()
    }
}