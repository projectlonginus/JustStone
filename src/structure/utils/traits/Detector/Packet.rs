use crate::structure::utils::{
    enums::{
        Packet,
        StoneTransferProtocol,
    },
    enums::PacketError,
    structs::define::{
        StructStoneHeader,
        StructStonePayload,
    },
    traits::define::Detector,
};
use crate::structure::utils::structs::define::EncryptionInfo;

impl Detector for Packet {
    fn display(&self) {
        self.payload().display()
    }

    fn get_type(&self) -> StoneTransferProtocol {
        self.payload().get_type()
    }

    fn get_size(&self) -> usize {
        self.payload().get_size()
    }

    fn get_encryption(&mut self) -> EncryptionInfo {
        self.payload().get_encryption()
    }

    fn get_header(&mut self) -> StructStoneHeader {
        self.mutable_payload().get_header()
    }

    fn get_payload(&mut self) -> StructStonePayload {
        self.mutable_payload().get_payload()
    }

    fn get_sysinfo(&mut self) -> Vec<u8> {
        self.mutable_payload().get_sysinfo()
    }

    fn get_command(&mut self) -> Vec<u8> {
        self.mutable_payload().get_command()
    }

    fn get_response(&mut self) -> Vec<u8> {
        self.mutable_payload().get_response()
    }

    fn get_file(&mut self) -> Vec<u8> {
        self.mutable_payload().get_file()
    }

    fn get_stone(&mut self) -> Option<Vec<u8>> {
        self.mutable_payload().get_stone()
    }

    fn take_header(&self) -> Option<&StructStoneHeader> {
        self.payload().take_header()
    }

    fn take_payload(&self) -> Option<&StructStonePayload> {
        self.payload().take_payload()
    }

    fn take_sysinfo(&self) -> Option<&Vec<u8>> {
        self.payload().take_sysinfo()
    }

    fn take_command(&self) -> Option<&Vec<u8>> {
        self.payload().take_command()
    }

    fn take_response(&self) -> Option<&Vec<u8>> {
        self.payload().take_response()
    }

    fn take_file(&self) -> Option<&Vec<u8>> {
        self.payload().take_file()
    }

    fn take_stone(&self) -> Option<&Vec<u8>> {
        self.payload().take_stone()
    }

    fn is_compression(&self) -> bool {
        self.payload().is_compression()
    }
    fn is_encryption(&self) -> bool {
        self.payload().is_encryption()
    }
}