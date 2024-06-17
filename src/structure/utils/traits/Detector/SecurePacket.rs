use std::fmt::Write;

use crate::structure::{
    enums::{
        StatusCode,
        StoneTransferProtocol,
    },
    structs::define::{
        SecurePacket,
        StructStoneHeader,
        StructStonePayload,
    },
    traits::define::Detector,
};

impl Detector for SecurePacket {
    fn display(&self) {
        let mut output = String::new();
        let header = &self.encrypted_packet.header;
        let payload = &self.encrypted_packet.payload;
        writeln!(output, "
            Header:
                Status: {:?}
                Type:   {:?}
                Size:   {:?}
            Payload:
                System information: {:?}
                Command input:      {:?}
                Response:           {:?}
                file:               {:?}",
                 StatusCode::type_check(&header.stone_status),
                 StoneTransferProtocol::type_check(&header.stone_type),
                 self.get_size(),
                 payload.sysinfo,
                 payload.command_input,
                 payload.response,
                 payload.file).unwrap();
        print!("{}", output)
    }

    fn get_type(&self) -> StoneTransferProtocol {
        StoneTransferProtocol::type_check(&self.encrypted_packet.header.stone_type)
    }

    fn get_size(&self) -> usize {
        let length_bytes: &[u8] = &self.encrypted_packet.header.stone_size;
        let length = u32::from_le_bytes([
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
        ]) + u32::from_le_bytes([
            length_bytes[4],
            length_bytes[5],
            0,
            0
        ]);
        return length as usize;
    }

    fn take_sysinfo(&self) -> Option<&Vec<u8>> { Option::from(&self.encrypted_packet.payload.sysinfo) }
    fn take_command(&self) -> Option<&Vec<u8>> { Option::from(&self.encrypted_packet.payload.command_input) }
    fn take_response(&self) -> Option<&Vec<u8>> { Option::from(&self.encrypted_packet.payload.response) }
    fn take_file(&self) -> Option<&Vec<u8>> { Option::from(&self.encrypted_packet.payload.file) }
    fn get_sysinfo(&self) -> Vec<u8> { self.encrypted_packet.payload.sysinfo.clone() }
    fn get_command(&self) -> Vec<u8> {
        self.encrypted_packet.payload.command_input.clone()
    }
    fn get_response(&self) -> Vec<u8> {
        self.encrypted_packet.payload.response.clone()
    }
    fn get_file(&self) -> Vec<u8> {
        self.encrypted_packet.payload.file.clone()
    }
    fn take_header(&self) -> Option<&StructStoneHeader> {
        Option::from(&self.encrypted_packet.header)
    }
    fn take_payload(&self) -> Option<&StructStonePayload> {
        Option::from(&self.encrypted_packet.payload)
    }
    fn get_header(&self) -> StructStoneHeader {
        self.encrypted_packet.header.clone()
    }
    fn get_payload(&self) -> StructStonePayload { self.encrypted_packet.payload.clone() }
    fn get_stone(&self) -> Option<&[u8]> {
        Option::from(self.encrypted_packet.stone.as_slice())
    }
    fn take_stone(&self) -> Option<&[u8]> {
        Option::from(self.encrypted_packet.stone.as_slice())
    }
    fn is_compression(&self) -> bool {
        self.encrypted_packet.header.is_compression()
    }
    fn is_encrypted(&self) -> bool {
        self.encrypted_packet.header.is_encrypted()
    }
}