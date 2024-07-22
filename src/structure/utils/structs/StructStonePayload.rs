use crate::structure::utils::structs::define::StructStonePayload;
use crate::utility::interface::utils::SystemInterface;

impl StructStonePayload {
    pub fn default() -> StructStonePayload {
        StructStonePayload {
            sysinfo: SystemInterface::info().as_bytes().to_vec(),
            command_input: vec![],
            response: vec![],
            file: vec![],
        }
    }
    pub fn from(sysinfo: Vec<u8>, command_input: Vec<u8>, response: Vec<u8>, file: Vec<u8>) -> StructStonePayload {
        StructStonePayload {
            sysinfo,
            command_input,
            response,
            file,
        }
    }

    pub fn new() -> StructStonePayload {
        StructStonePayload {
            sysinfo: vec![],
            command_input: vec![],
            response: vec![],
            file: vec![],
        }
    }

    pub fn get_size(&self) -> usize {
        return self.sysinfo.len()
            + self.command_input.len()
            + self.response.len()
            + self.file.len();
    }

    pub fn is_empty(&self) -> bool {
        if self.get_size() == 0 {
            return true;
        }
        false
    }

    pub fn get_non_empty_data(&self) -> Vec<u8> {
        let mut non_empty_vectors: i32 = 0;
        if !self.command_input.is_empty() {
            non_empty_vectors = 1;
        }
        if !self.response.is_empty() {
            non_empty_vectors = 2;
        }
        if !self.file.is_empty() {
            non_empty_vectors = 3;
        }
        match non_empty_vectors {
            1 => self.command_input.clone(),
            2 => self.response.clone(),
            3 => self.file.clone(),
            _ => vec![]
        }
    }
}