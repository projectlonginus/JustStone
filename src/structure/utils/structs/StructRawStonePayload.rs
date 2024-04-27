use crate::structure::structs::define::StructRawStonePayload;

impl StructRawStonePayload {
    pub fn from(sysinfo: String, command_input: String, response: String, file: String) -> StructRawStonePayload {
        StructRawStonePayload {
            sysinfo,
            command_input,
            response,
            file,
        }
    }

    pub fn new() -> StructRawStonePayload {
        StructRawStonePayload {
            sysinfo: String::new(),
            command_input: String::new(),
            response: String::new(),
            file: String::new(),
        }
    }
}