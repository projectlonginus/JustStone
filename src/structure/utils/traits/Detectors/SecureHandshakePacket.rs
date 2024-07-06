use std::fmt::Write;
use std::mem::replace;

use crate::structure::utils::{
    enums::{
        EncryptionFlag,
        StatusCode,
        StoneTransferProtocol
    },
    structs::define::{
        EncryptionInfo,
        SecureHandshakePacket,
        StructStoneHeader,
        StructStonePayload
    },
    traits::define::{
        Detector,
        ProtocolCodec
    }
};

impl Detector for SecureHandshakePacket {
    fn display(&self) {
        let mut output = String::new();
        writeln!(output, "\
        SecureHandshakePacket:
        Encryption Flag:             {:?} ({:?})
        Encrypted Data Block Length: {:?} ({:?})
        Encrypted Data Field: \
        ", EncryptionFlag::get_type(&self.encryption_flag), self.encryption_flag,
        self.get_size(), self.encrypt_data_block_length.to_be_bytes()
        ).unwrap();
        print!("{}", output);
        self.origin_packet.display()
    }
    fn get_status(&self) -> StatusCode { StatusCode::get_type(&self.origin_packet.header.stone_status) }
    fn get_type(&self) -> StoneTransferProtocol { StoneTransferProtocol::get_type(&self.origin_packet.header.stone_type) }
    fn get_size(&self) -> usize { self.origin_packet.get_size() + 12 }
    fn get_encryption(&self) -> EncryptionInfo { EncryptionFlag::get_type(&self.encryption_flag).get_types() }
    fn get_header(&mut self) -> StructStoneHeader { replace(&mut self.origin_packet.header, Default::default()) }
    fn get_payload(&mut self) -> StructStonePayload { replace(&mut self.origin_packet.payload, Default::default()) }
    fn get_sysinfo(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.sysinfo, Default::default()) }
    fn get_command(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.command_input, Default::default()) }
    fn get_response(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.response, Default::default()) }
    fn get_file(&mut self) -> Vec<u8> { replace(&mut self.origin_packet.payload.file, Default::default()) }
    fn get_stone(&mut self) -> Option<Vec<u8>> { Option::from(replace(&mut self.encrypted_packet, Default::default())) }
    fn take_header(&self) -> Option<&StructStoneHeader> { Option::from(&self.origin_packet.header) }
    fn take_payload(&self) -> Option<&StructStonePayload> { Option::from(&self.origin_packet.payload) }
    fn take_sysinfo(&self) -> Option<&Vec<u8>> { Option::from(&self.origin_packet.payload.sysinfo) }
    fn take_command(&self) -> Option<&Vec<u8>> { Option::from(&self.origin_packet.payload.command_input) }
    fn take_response(&self) -> Option<&Vec<u8>> { Option::from(&self.origin_packet.payload.response) }
    fn take_file(&self) -> Option<&Vec<u8>> { Option::from(&self.origin_packet.payload.file) }
    fn take_stone(&self) -> Option<&Vec<u8>> { Option::from(&self.encrypted_packet) }
    fn is_compression(&self) -> bool { self.origin_packet.header.is_compression() }
    fn is_encryption(&self) -> bool { self.origin_packet.header.is_signed() }
}