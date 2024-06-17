use crate::{
    stprotocol::Session,
    structure::utils::{
        enums::EncryptType::NotEncryption,
        traits::define::Detector,
    }
    ,
};
use crate::structure::utils::enums::StoneTransferProtocol;
use crate::structure::utils::structs::define::{StructStoneHeader, StructStonePayload};

impl Detector for Session {
    fn display(&self) {
        todo!()
    }

    fn get_type(&self) -> StoneTransferProtocol {
        todo!()
    }

    fn get_size(&self) -> usize {
        todo!()
    }

    fn take_sysinfo(&self) -> Option<&Vec<u8>> {
        todo!()
    }

    fn take_command(&self) -> Option<&Vec<u8>> {
        todo!()
    }

    fn take_response(&self) -> Option<&Vec<u8>> {
        todo!()
    }

    fn take_file(&self) -> Option<&Vec<u8>> {
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

    fn take_header(&self) -> Option<&StructStoneHeader> {
        todo!()
    }

    fn take_payload(&self) -> Option<&StructStonePayload> {
        todo!()
    }

    fn get_header(&self) -> StructStoneHeader {
        todo!()
    }

    fn get_payload(&self) -> StructStonePayload {
        todo!()
    }

    fn get_stone(&self) -> Option<&[u8]> {
        todo!()
    }

    fn take_stone(&self) -> Option<&[u8]> {
        todo!()
    }

    fn is_compression(&self) -> bool {
        todo!()
    }

    fn is_encryption(&self) -> bool {
        if self.encryption == NotEncryption {
            false
        } else {
            true
        }
    }
}