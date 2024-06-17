use crate::stprotocol::Client;
use crate::structure::utils::enums::StoneTransferProtocol;
use crate::structure::utils::structs::define::{StructStoneHeader, StructStonePayload};
use crate::structure::utils::traits::define::Detector;

impl Detector for Client {
    fn take_sysinfo(&self) -> Option<&Vec<u8>> { self.session.take_packet().take_sysinfo() }

    fn take_command(&self) -> Option<&Vec<u8>> {
        self.session.take_packet().take_command()
    }

    fn take_response(&self) -> Option<&Vec<u8>> {
        self.session.take_packet().take_response()
    }

    fn take_file(&self) -> Option<&Vec<u8>> {
        self.session.take_file()
    }

    fn get_sysinfo(&self) -> Vec<u8> { self.session.get_sysinfo() }

    fn get_command(&self) -> Vec<u8> {
        self.session.get_command()
    }

    fn get_response(&self) -> Vec<u8> {
        self.session.get_response()
    }

    fn get_file(&self) -> Vec<u8> {
        self.session.get_file()
    }
    fn display(&self) {
        self.session.display()
    }

    fn get_type(&self) -> StoneTransferProtocol {
        self.session.get_type()
    }

    fn get_size(&self) -> usize {
        self.session.get_size()
    }

    fn take_header(&self) -> Option<&StructStoneHeader> {
        self.session.take_header()
    }

    fn take_payload(&self) -> Option<&StructStonePayload> {
        self.session.take_payload()
    }

    fn get_header(&self) -> StructStoneHeader {
        self.session.get_header()
    }

    fn get_payload(&self) -> StructStonePayload {
        self.session.get_payload()
    }

    fn get_stone(&self) -> Option<&[u8]> {
        self.session.get_stone()
    }

    fn take_stone(&self) -> Option<&[u8]> {
        self.session.take_stone()
    }

    fn is_compression(&self) -> bool {
        self.session.is_compression()
    }

    fn is_encryption(&self) -> bool {
        self.session.is_encryption()
    }
}