use json::{JsonValue, object};

use crate::structure::{
    enums::{
        StatusCode,
        StoneTransferProtocol,
    },
    packet::StructStonePayload::PACKET_DELIMITER,
    structs::define::{
        StructRawStonePayload,
        StructStoneHeader,
        StructStonePayload,
    },
    traits::define::TypeManager,
    utils::traits::define::ProtocolCodec,
};

impl TypeManager for StructRawStonePayload {
    fn to_json(&self) -> JsonValue {
        return object! {
            sysinfo: self.sysinfo.clone(),
            command_input: self.command_input.clone(),
            response: self.response.clone(),
            file: self.file.clone()
        };
    }

    fn to_vec(&self) -> Vec<u8> {
        let sysinfo = self.sysinfo.as_bytes().to_vec();
        let command_input = self.command_input.as_bytes().to_vec();
        let response = self.response.as_bytes().to_vec();
        let file = self.file.as_bytes().to_vec();

        StructStonePayload::from(sysinfo, command_input, response, file).to_vec()
    }
}

impl TypeManager for StructStonePayload {
    fn to_json(&self) -> JsonValue {
        return object! {
            sysinfo: String::from_utf8(self.sysinfo.clone()).unwrap(),
            command_input: String::from_utf8(self.command_input.clone()).unwrap(),
            response: String::from_utf8(self.response.clone()).unwrap(),
            file: String::from_utf8(self.file.clone()).unwrap()
        };
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.take_sysinfo());
        payload.extend(PACKET_DELIMITER);
        payload.extend(self.take_command_input());
        payload.extend(PACKET_DELIMITER);
        payload.extend(self.take_response());
        payload.extend(PACKET_DELIMITER);
        payload.extend(self.take_file());
        payload.extend(PACKET_DELIMITER);
        payload
    }
}

impl TypeManager for StructStoneHeader {
    fn to_json(&self) -> JsonValue {
        let mut array = [0; std::mem::size_of::<usize>()];
        array.copy_from_slice(&self.stone_size);

        return object! {
            stone_status: StatusCode::type_check(self.take_stone_status()).to_string(),
            stone_type: StoneTransferProtocol::type_check(self.take_stone_type()).to_string(),
            stone_size: usize::from_le_bytes(array)
        };
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.extend(self.take_stone_status());
        header.extend(self.take_stone_type());
        header.extend(self.take_stone_size());
        header
    }
}

