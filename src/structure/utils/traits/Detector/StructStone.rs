use std::{
    fmt::Write,
    mem::replace
};

use crate::{
    structure::{
        utils::structs::define::EncryptionInfo,
        utils::{
            enums::{StatusCode, StoneTransferProtocol},
            structs::define::{StructStone, StructStoneHeader, StructStonePayload},
            traits::define::Detector,
        },
        utils::traits::define::ProtocolCodec
    }
};

impl Detector for StructStone {
    fn display(&self) {
        let mut output = String::new();
        writeln!(output, "\
        Header:
        Status: {:?} ({:?})
        Type:   {:?} ({:?})
        Size:   {:?}\n\
        Payload:
        System information: {:?}
        Command input:      {:?}
        Response:           {:?}
        file:               {:?}",
                 self.get_status(), self.header.stone_status,
                 self.get_type(), self.header.stone_type,
                 self.get_size(),
                 self.payload.sysinfo,
                 self.payload.command_input,
                 self.payload.response,
                 self.payload.file
        ).unwrap();
        print!("{}", output);
        output.clear()
    }

    fn get_status(&self) -> StatusCode {
        StatusCode::get_type(&self.header.stone_status)
    }

    fn get_type(&self) -> StoneTransferProtocol {
        StoneTransferProtocol::get_type(&self.header.stone_type)
    }
    fn get_size(&self) -> usize {
        self.header.stone_size as usize
    }

    fn get_encryption(&self) -> EncryptionInfo {
        EncryptionInfo::default()
    }

    fn get_header(&mut self) -> StructStoneHeader { replace(&mut self.header, Default::default()) }
    fn get_payload(&mut self) -> StructStonePayload { replace(&mut self.payload, Default::default()) }
    fn get_sysinfo(&mut self) -> Vec<u8> { replace(&mut self.payload.sysinfo, Default::default()) }
    fn get_command(&mut self) -> Vec<u8> { replace(&mut self.payload.command_input, Default::default()) }
    fn get_response(&mut self) -> Vec<u8> { replace(&mut self.payload.response, Default::default()) }
    fn get_file(&mut self) -> Vec<u8> { replace(&mut self.payload.file, Default::default()) }
    fn get_stone(&mut self) -> Option<Vec<u8>> { Option::from(replace(&mut self.stone, Default::default())) }
    fn take_header(&self) -> Option<&StructStoneHeader> { Option::from(&self.header) }
    fn take_payload(&self) -> Option<&StructStonePayload> { Option::from(&self.payload) }
    fn take_sysinfo(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.sysinfo) }
    fn take_command(&self) -> Option<&Vec<u8>> {
        Option::from(&self.payload.command_input)
    }
    fn take_response(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.response) }
    fn take_file(&self) -> Option<&Vec<u8>> { Option::from(&self.payload.file) }
    fn take_stone(&self) -> Option<&Vec<u8>> {
        Option::from(&self.stone)
    }
    fn is_compression(&self) -> bool {
        self.header.is_compression()
    }
    fn is_encryption(&self) -> bool {
        self.header.is_signed()
    }
}