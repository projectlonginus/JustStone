use std::fmt::{Debug, Write};

use crate::structure::{
    enums::{StatusCode, StoneTransferProtocol},
    structs::define::{StructStone, StructStoneHeader, StructStonePayload},
    traits::define::Detector,
};

impl Detector for StructStone {
    fn display(&self) {
        let mut output = String::new();
        writeln!(output, "\
        Header: \n\
            Status: {:?}\n\
            Type:   {:?}\n\
            Size:   {:?}\n\
        Payload: \n\
            System information: {:?}\n\
            Command input:      {:?}\n\
            Response:           {:?}\n\
            file:               {:?}\n",
                 StatusCode::type_check(&self.header.stone_status),
                 StoneTransferProtocol::type_check(&self.header.stone_type),
                 self.get_size(),
                 self.payload.sysinfo,
                 self.payload.command_input,
                 self.payload.response,
                 self.payload.file).unwrap();
        print!("{}", output)
    }
    fn get_type(&self) -> StoneTransferProtocol {
        StoneTransferProtocol::type_check(&self.header.stone_type)
    }
    fn get_size(&self) -> usize {
        let length_bytes: &[u8] = &self.header.stone_size;
        let length = u32::from_le_bytes([
            length_bytes[0],
            length_bytes[1],
            length_bytes[2],
            length_bytes[3],
        ]);
        return length as usize;
    }
    fn take_sysinfo(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.sysinfo) }
    fn take_command(&self) -> Option<&Vec<u8>> {
        Option::from(&self.payload.command_input)
    }
    fn take_response(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.response) }
    fn take_file(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.file) }
    fn get_sysinfo(&self) -> Vec<u8> { self.payload.sysinfo.clone() }
    fn get_command(&self) -> Vec<u8> { self.payload.command_input.clone() }
    fn get_response(&self) -> Vec<u8> { self.payload.response.clone() }
    fn get_file(&self) -> Vec<u8> { self.payload.file.clone() }
    fn take_header(&self) -> Option<&StructStoneHeader> { Option::from(&self.header) }
    fn take_payload(&self) -> Option<&StructStonePayload> { Option::from(&self.payload) }
    fn get_header(&self) -> StructStoneHeader { self.header.clone() }
    fn get_payload(&self) -> StructStonePayload { self.payload.clone() }
    fn get_stone(&self) -> Option<&[u8]> { Option::from(self.stone.as_slice()) }
    fn take_stone(&self) -> Option<&[u8]> {
        Option::from(self.stone.as_slice())
    }
    fn is_compression(&self) -> bool {
        self.header.is_compression()
    }
    fn is_encrypted(&self) -> bool {
        self.header.is_encrypted()
    }
}