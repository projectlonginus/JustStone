use std::fmt::{Debug, Write};

use json::{JsonValue, object};
use sysinfo::System;

use crate::structure::packet::PACKET_DELIMITER;

pub struct SecureHandshakePacket {
    encrypt_data_block_length: Vec<u8>,
    handshake_tpye: Vec<u8>,
    encrypt_type: Vec<u8>,
    encrypted_packet: Vec<u8>
}

pub struct SecurePacket {
    encrypt_data_block_length: Vec<u8>,
    encrypted_packet: Vec<u8>
}

pub struct StructRawStonePayload {
    sysinfo: String,
    command_input: String,
    response: String,
    file: String,
}

#[derive(Debug, Clone)]
pub struct StructStonePayload {
    sysinfo: Vec<u8>,
    command_input: Vec<u8>,
    response: Vec<u8>,
    file: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct StructStoneHeader {
    stone_status: Vec<u8>,
    stone_type: Vec<u8>,
    stone_size: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct StructStone {
    header: StructStoneHeader,
    payload: StructStonePayload,
    stone: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoneTransferProtocol {
    Connection,
    Handshake,
    Request,
    Response,
    HealthCheck,
    Disconnect,

    ExecuteCmd,
    Upload,
    Download,

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
    Normal,
    // 압축 x 암호화 x
    Compressed,
    // 압축 o 암호화 x
    Secured,
    // 압축 x 암호화 o
    SCPacket,
    // 압축 o 암호화 o
    Modulated,   // 패킷이 변조되거나 손상됨
}

#[derive(Debug, Clone)]
pub struct PacketBuilder {
    compression: bool,
    protocol: StoneTransferProtocol,
    output: StructStonePayload,
}

pub trait ProtocolCodec {
    fn to_vec(&self) -> Vec<u8>;
    fn to_string(&self) -> String;
}

impl PacketBuilder {
    pub fn is_compression(&self) -> &bool {
        &self.compression
    }

    pub fn protocol(&self) -> &StoneTransferProtocol {
        &self.protocol
    }

    pub fn output(&self) -> StructStonePayload {
        self.output.clone()
    }

    pub fn default() -> PacketBuilder {
        PacketBuilder {
            compression: false,
            protocol: StoneTransferProtocol::Unknown,
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        compression: bool,
        protocol: StoneTransferProtocol,
        output: StructStonePayload,
    ) -> PacketBuilder {
        PacketBuilder {
            compression,
            protocol,
            output,
        }
    }
}

pub trait TypeManager {
    fn to_json(&self) -> JsonValue;
    fn to_vec(&self) -> Vec<u8>;
}

impl TypeManager for StructRawStonePayload {
    fn to_json(&self) -> JsonValue {
        return object! {
            sysinfo: self.sysinfo.clone(),
            command_input: self.command_input.clone(),
            response: self.response.clone(),
            file: self.file.clone()
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let sysinfo = self.sysinfo.as_bytes().to_vec();
        let command_input = self.command_input.as_bytes().to_vec();
        let response = self.response.as_bytes().to_vec();
        let file = self.file.as_bytes().to_vec();

        StructStonePayload::from(sysinfo, command_input, response, file).to_vec()
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
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.extend(self.take_stone_status());
        header.extend(self.take_stone_type());
        header.extend(self.take_stone_size());
        header
    }
}

impl TypeManager for StructStonePayload {
    fn to_json(&self) -> JsonValue {
        return object! {
            sysinfo: String::from_utf8(self.sysinfo.clone()).unwrap(),
            command_input: String::from_utf8(self.command_input.clone()).unwrap(),
            response: String::from_utf8(self.response.clone()).unwrap(),
            file: String::from_utf8(self.file.clone()).unwrap()
        }
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

pub trait Detector {
    fn display(&self);
    fn get_type(&self) -> StoneTransferProtocol;
    fn get_size(&self) -> usize;
    fn take_sysinfo(&self) -> &Vec<u8>;
    fn take_command(&self) -> &Vec<u8>;
    fn take_response(&self) -> &Vec<u8>;
    fn take_file(&self) -> &Vec<u8>;
    fn get_sysinfo(&self) -> Vec<u8>;
    fn get_command(&self) -> Vec<u8>;
    fn get_response(&self) -> Vec<u8>;
    fn get_file(&self) -> Vec<u8>;
    fn take_header(&self) -> &StructStoneHeader;
    fn take_payload(&self) -> &StructStonePayload;
    fn get_header(&self) -> StructStoneHeader;
    fn get_payload(&self) -> StructStonePayload;
    fn get_stone(&self) -> &[u8];
    fn is_compression(&self) -> bool;
}

impl Detector for StructStone {
    fn display(&self) {
        let mut output = String::new();
        writeln!(output, "Header: \n    \
        Status: {:?}\n    Type: {:?}\n    Size: {:?}\n\
        Payload: \n    System information: {:?}\n    Command input:    {:?}\n    Response:    {:?}\
        \n    file:    {:?}\n",
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
    fn take_sysinfo(&self) -> &Vec<u8> { &self.payload.sysinfo }
    fn take_command(&self) -> &Vec<u8> {
        &self.payload.command_input
    }
    fn take_response(&self) -> &Vec<u8> { &self.payload.response }
    fn take_file(&self) -> &Vec<u8> { &self.payload.file }
    fn get_sysinfo(&self) -> Vec<u8> { self.payload.sysinfo.clone() }
    fn get_command(&self) -> Vec<u8> { self.payload.command_input.clone() }
    fn get_response(&self) -> Vec<u8> { self.payload.response.clone() }
    fn get_file(&self) -> Vec<u8> { self.payload.file.clone() }
    fn take_header(&self) -> &StructStoneHeader { &self.header }
    fn take_payload(&self) -> &StructStonePayload { &self.payload }
    fn get_header(&self) -> StructStoneHeader { self.header.clone() }
    fn get_payload(&self) -> StructStonePayload { self.payload.clone() }
    fn get_stone(&self) -> &[u8] { self.stone.as_slice() }
    fn is_compression(&self) -> bool {
        self.header.is_compression()
    }
}

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

impl StructStoneHeader {
    pub fn from(stone_status: Vec<u8>, stone_type: Vec<u8>, stone_size: Vec<u8>) -> StructStoneHeader {
        StructStoneHeader {
            stone_status,
            stone_type,
            stone_size,
        }
    }

    pub fn new() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![],
            stone_type: vec![],
            stone_size: vec![],
        }
    }

    pub fn set_stone_status(&mut self, stone_status: Vec<u8>) {
        self.stone_status = stone_status;
    }

    pub fn set_stone_type(&mut self, stone_type: Vec<u8>) {
        self.stone_type = stone_type;
    }

    pub fn set_stone_size(&mut self, stone_size: Vec<u8>) {
        self.stone_size = stone_size;
    }

    pub fn take_stone_status(&self) -> &Vec<u8> {
        &self.stone_status
    }

    pub fn take_stone_type(&self) -> &Vec<u8> {
        &self.stone_type
    }

    pub fn take_stone_size(&self) -> &Vec<u8> {
        &self.stone_size
    }
    pub fn is_compression(&self) -> bool {
        match self.stone_status[..] {
            [1, 0, 0, 0] => true,
            _ => false
        }
    }

    pub fn default() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![0, 0, 0, 0],
            stone_type: vec![0, 0, 0, 0],
            stone_size: vec![12, 0, 0, 0],
        }
    }
}

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
        let mut non_empty_vectors:i32 = 0;
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

    pub fn default() -> StructStonePayload {
        StructStonePayload {
            sysinfo: sysinfo().as_bytes().to_vec(),
            command_input: vec![],
            response: vec![],
            file: vec![],
        }
    }
}


impl StructStone {
    pub fn set(&mut self, source: StructStone) {
        self.header = source.header;
        self.payload = source.payload;
        self.stone = source.stone;
    }
    pub fn set_header(&mut self, stone_status: Vec<u8>, stone_type: Vec<u8>, stone_size: Vec<u8>) {
        self.header.stone_status = stone_status;
        self.header.stone_type = stone_type;
        self.header.stone_size = stone_size;
    }
    pub fn set_payload(&mut self, sys_info: Vec<u8>, command: Vec<u8>, response: Vec<u8>, file: Vec<u8>) {
        self.payload.sysinfo = sys_info;
        self.payload.command_input = command;
        self.payload.response = response;
        self.payload.file = file;
    }

    pub fn set_stone_status(&mut self, stone_status: Vec<u8>) {
        self.header.stone_status = stone_status;
    }
    pub fn set_stone_type(&mut self, stone_type: Vec<u8>) {
        self.header.stone_type = stone_type;
    }

    pub fn set_stone_size(&mut self, stone_size: Vec<u8>) {
        self.header.stone_size = stone_size;
    }
    pub fn from(header: StructStoneHeader, payload: StructStonePayload, stone: Vec<u8>) -> StructStone {
        StructStone {
            header,
            payload,
            stone,
        }
    }
    pub fn new() -> StructStone {
        StructStone {
            header: StructStoneHeader::new(),
            payload: StructStonePayload::new(),
            stone: vec![],
        }
    }
}

pub fn sysinfo() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    format!(
        "
        [ system information ]

        total memory: {} bytes
        used memory : {} bytes
        total swap  : {} bytes
        used swap   : {} bytes

        System global_cpu_info:  {:?}
        System name              {:?}
        System kernel version:   {:?}
        System OS version:       {:?}
        System host name:        {:?}",
        sys.total_memory(),
        sys.used_memory(),
        sys.total_swap(),
        sys.used_swap(),
        sys.global_cpu_info(),
        System::name(),
        System::kernel_version(),
        System::os_version(),
        System::host_name(),
    )
}
