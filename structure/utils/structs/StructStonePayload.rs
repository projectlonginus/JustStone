use crate::structure::structs::define::StructStonePayload;

impl StructStonePayload {
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

    pub fn take_sysinfo(&self) -> &Vec<u8> {
        &self.sysinfo
    }

    pub fn take_command_input(&self) -> &Vec<u8> {
        &self.command_input
    }

    pub fn take_response(&self) -> &Vec<u8> {
        &self.response
    }

    pub fn take_file(&self) -> &Vec<u8> {
        &self.file
    }
}