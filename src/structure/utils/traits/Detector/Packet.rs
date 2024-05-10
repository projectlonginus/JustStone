use std::fmt::Debug;

use crate::structure::{
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
use crate::structure::structs::define::StructStone;

impl Detector for Packet {
    fn display(&self) {
        if let Some(payload) = self.payload() {
            payload.display();
        } else {
            println!("{:?}", PacketError::UnexpectedError("UnexpectedError".to_string()));
        }
    }

    fn get_type(&self) -> StoneTransferProtocol {
        if let Some(payload) = self.payload() {
            payload.get_type()
        } else {
            StoneTransferProtocol::Unknown
        }
    }

    fn get_size(&self) -> usize {
        if let Some(payload) = self.payload() {
            payload.get_size()
        } else {
            0
        }
    }

    fn take_sysinfo(&self) -> &Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.take_sysinfo()
        } else {
            &vec![]
        }
    }

    fn take_command(&self) -> &Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.take_command()
        } else {
            &vec![]
        }
    }

    fn take_response(&self) -> &Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.take_response()
        } else {
            &vec![]
        }
    }

    fn take_file(&self) -> &Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.take_file()
        } else {
            &vec![]
        }
    }

    fn get_sysinfo(&self) -> Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.get_sysinfo()
        } else {
            vec![]
        }
    }

    fn get_command(&self) -> Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.get_command()
        } else {
            vec![]
        }
    }

    fn get_response(&self) -> Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.get_response()
        } else {
            vec![]
        }
    }

    fn get_file(&self) -> Vec<u8> {
        if let Some(payload) = self.payload() {
            payload.get_file()
        } else {
            vec![]
        }
    }

    fn take_header(&self) -> &StructStoneHeader {
        if let Some(payload) = self.payload() {
            payload.take_header()
        } else {
            &StructStoneHeader::new()
        }
    }

    fn take_payload(&self) -> &StructStonePayload {
        if let Some(payload) = self.payload() {
            payload.take_payload()
        } else {
            &StructStonePayload::new()
        }
    }

    fn get_header(&self) -> StructStoneHeader {
        if let Some(payload) = self.payload() {
            payload.get_header()
        } else {
            StructStoneHeader::new()
        }
    }

    fn get_payload(&self) -> StructStonePayload {
        if let Some(payload) = self.payload() {
            payload.get_payload()
        } else {
            StructStonePayload::new()
        }
    }

    fn get_stone(&self) -> &[u8] {
        if let Some(payload) = self.payload() {
            payload.get_stone()
        } else {
            StructStone::new().get_stone()
        }
    }

    fn take_stone(&self) -> &[u8] {
        if let Some(payload) = self.payload() {
            payload.take_stone()
        } else {
            &StructStone::new().take_stone()
        }
    }

    fn is_compression(&self) -> bool {
        if let Some(payload) = self.payload() {
            payload.is_compression()
        } else {
            false
        }
    }
    fn is_encrypted(&self) -> bool {
        if let Some(payload) = self.payload() {
            payload.is_encrypted()
        } else {
            false
        }
    }
}