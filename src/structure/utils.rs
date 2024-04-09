use json::{object, JsonValue};
use std::fmt::{Debug, Write};
use sysinfo::System;

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

#[derive(Debug, Clone)]
pub struct PacketBuilder {
    encipher: bool,
    protocol: StoneTransferProtocol,
    output: StructStonePayload,
}

impl PacketBuilder {
    pub fn encipher(&self) -> &bool {
        &self.encipher
    }

    pub fn protocol(&self) -> &StoneTransferProtocol {
        &self.protocol
    }

    pub fn output(&self) -> StructStonePayload {
        self.output.clone()
    }

    pub fn default() -> PacketBuilder {
        PacketBuilder {
            encipher: false,
            protocol: StoneTransferProtocol::Unknown,
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        encipher: bool,
        protocol: StoneTransferProtocol,
        output: StructStonePayload,
    ) -> PacketBuilder {
        PacketBuilder {
            encipher,
            protocol,
            output,
        }
    }
}

pub trait TypeManager {
    fn deserialization<T>(&self) -> T
        where
            T: Debug
            + Sized
            + Clone
            + From<Vec<u8>>
            + From<StructStonePayload>
            + From<StructStoneHeader>
            + From<StructStone>
            + From<JsonValue>
            + From<String>;

    fn serialization<T>(&self) -> T
        where
            T: Debug
            + Sized
            + Clone
            + From<Vec<u8>>
            + From<StructStonePayload>
            + From<StructStoneHeader>
            + From<StructStone>
            + From<JsonValue>
            + From<String>;
}

impl TypeManager for StructRawStonePayload {
    fn deserialization<T>(&self) -> T
        where
            T: From<JsonValue>,
    {
        return object! {
            sysinfo: self.sysinfo.clone(),
            command_input: self.command_input.clone(),
            response: self.response.clone(),
            file: self.file.clone()
        }
            .into();
    }

    fn serialization<T>(&self) -> T
        where
            T: From<StructStonePayload>,
    {
        let sysinfo = self.sysinfo.as_bytes().to_vec();
        let command_input = self.command_input.as_bytes().to_vec();
        let response = self.response.as_bytes().to_vec();
        let file = self.file.as_bytes().to_vec();

        StructStonePayload {
            sysinfo,
            command_input,
            response,
            file,
        }
            .into()
    }
}

impl TypeManager for StructStoneHeader {
    fn deserialization<T>(&self) -> T
        where
            T: From<JsonValue>,
    {
        let mut array = [0; std::mem::size_of::<usize>()];
        array.copy_from_slice(&self.stone_size);

        return object! {
            stone_status: if self.stone_status == vec![0,0,0,0] {
                    false
                } else { true },
            stone_type: StoneTransferProtocol::type_check(&self.stone_type).to_string(),
            stone_size: usize::from_le_bytes(array),
        }
            .into();
    }

    fn serialization<T>(&self) -> T
        where
            T: Debug + From<StructStoneHeader>,
    {
        todo!()
    }
}

impl TypeManager for StructStonePayload {
    fn deserialization<T>(&self) -> T
        where
            T: From<JsonValue>,
    {
        return object! {
            sysinfo: String::from_utf8(self.sysinfo.clone()).unwrap(),
            command_input: String::from_utf8(self.command_input.clone()).unwrap(),
            response: String::from_utf8(self.response.clone()).unwrap(),
            file: String::from_utf8(self.file.clone()).unwrap()
        }
            .into();
    }

    fn serialization<T>(&self) -> T
        where
            T: Debug + From<StructStoneHeader>,
    {
        todo!()
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
}

impl Detector for StructStone {
    fn display(&self) {
        let mut output = String::new();
        writeln!(output, "Header: \n    Status: {:?}\n    Type: {:?}\n    Size: {:?}\nPayload: \n    System information: {:?}\n    Command input:    {:?}\n    Response:    {:?}\n    file:    {:?}\n",
                 self.header.stone_status,
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

    pub fn get_size(&mut self) -> usize {
        return self.sysinfo.len()
            + self.command_input.len()
            + self.response.len()
            + self.file.len();
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