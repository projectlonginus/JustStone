use bstr::ByteSlice;
use std::convert::AsRef;
use sysinfo::System;

// pub struct  StoneChain {
//     previous_stone_hash: Vec<u8>,
//     stone_hash:          Vec<u8>,
//     stonetree_hash:      Vec<u8>,
//     timestamp:           Vec<u8>,
//     transaction_list:    Vec<u8>
// }
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
    file: Vec<u8>, // pub stone_chain: StoneChain,
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
    Type: StoneTransferProtocol,
    output: StructStonePayload,
}

impl PacketBuilder {
    pub fn StructStone(&mut self) -> StructStone {
        StructStone::build(
            StructStoneHeader::build(self.encipher, self.Type.clone(), self.output.get_size()),
            self.output.clone(),
        )
    }
}

pub trait TypeManager {
    fn deserialization(&self) -> StoneTransferProtocol;
    fn serialization(&self) -> Vec<u8>;
}

impl TypeManager for StoneTransferProtocol {
    fn deserialization(&self) -> StoneTransferProtocol {
        todo!()
    }

    fn serialization(&self) -> Vec<u8> {
        match self {
            StoneTransferProtocol::Connection => vec![0, 0, 0, 0],
            StoneTransferProtocol::Handshake => vec![1, 0, 0, 0],
            StoneTransferProtocol::HealthCheck => vec![4, 0, 0, 0],
            StoneTransferProtocol::Disconnect => vec![5, 0, 0, 0],

            StoneTransferProtocol::ExecuteCmd => vec![2, 0, 0, 0],
            StoneTransferProtocol::Upload => vec![7, 0, 0, 0],
            StoneTransferProtocol::Download => vec![8, 0, 0, 0],
            StoneTransferProtocol::Response => vec![3, 0, 0, 0],

            StoneTransferProtocol::Unknown => vec![0, 0, 0, 1],
            _ => vec![0, 0, 0, 2],
        }
    }
}
pub trait Generator {
    fn to_stone(self) -> StructStone;
}

impl Generator for StructRawStonePayload {
    fn to_stone(self) -> StructStone {
        let ssp = StructRawStonePayload::serialization(&self);
        let ssh = StructStoneHeader::from(&ssp);

        StructStone::build(ssh, ssp)
    }
}

impl Generator for StructStonePayload {
    fn to_stone(self) -> StructStone {
        let ssh = StructStoneHeader::from(&self);

        StructStone::build(ssh, self)
    }
}

pub trait Detector {
    fn get_type(&self) -> StoneTransferProtocol;
    fn get_size(&self) -> usize;
    fn get_command(&self) -> Vec<u8>;
    fn get_file(&self) -> Vec<u8>;
    fn get_header(&self) -> StructStoneHeader;
    fn get_payload(&self) -> StructStonePayload;
    fn get_stone(&self) -> &[u8];
}

impl Detector for StructStone {
    fn get_type(&self) -> StoneTransferProtocol {
        match &self.header.stone_type.as_slice() {
            &[0, 0, 0, 0] => StoneTransferProtocol::Connection,
            &[1, 0, 0, 0] => StoneTransferProtocol::Handshake,
            &[4, 0, 0, 0] => StoneTransferProtocol::HealthCheck,
            &[5, 0, 0, 0] => StoneTransferProtocol::Disconnect,

            &[2, 0, 0, 0] => StoneTransferProtocol::ExecuteCmd,
            &[7, 0, 0, 0] => StoneTransferProtocol::Upload,
            &[8, 0, 0, 0] => StoneTransferProtocol::Download,
            &[3, 0, 0, 0] => StoneTransferProtocol::Response,

            _ => StoneTransferProtocol::Unknown,
        }
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
    fn get_command(&self) -> Vec<u8> {
        self.payload.command_input.clone()
    }
    fn get_file(&self) -> Vec<u8> {
        self.payload.file.clone()
    }
    fn get_header(&self) -> StructStoneHeader {
        self.header.clone()
    }
    fn get_payload(&self) -> StructStonePayload {
        self.payload.clone()
    }
    fn get_stone(&self) -> &[u8] {
        self.stone.as_slice()
    }
}

impl StructRawStonePayload {
    pub fn new(command_input: &str, response: &str, file: &str) -> StructRawStonePayload {
        StructRawStonePayload {
            sysinfo: StructRawStonePayload::sysinfo(),
            command_input: command_input.to_string(),
            response: response.to_string(),
            file: file.to_string(),
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
    pub fn serialization(&self) -> StructStonePayload {
        let sysinfo = self.sysinfo.as_bytes().to_vec();
        let command_input = self.command_input.as_bytes().to_vec();
        let response = self.response.as_bytes().to_vec();

        StructStonePayload {
            sysinfo,
            command_input,
            response,
            file: vec![],
        }
    }
}

impl StructStoneHeader {
    pub fn deserialization(packet: Vec<u8>) -> StructStoneHeader {
        if packet[0..4] != vec![0, 0, 0, 0] {
            return StructStoneHeader::default();
        } else {
            StructStoneHeader {
                stone_status: Vec::from(&packet[0..4]),
                stone_type: Vec::from(&packet[4..8]),
                stone_size: Vec::from(&packet[8..12]),
            }
        }
    }

    pub fn from(payload: &StructStonePayload) -> StructStoneHeader {
        let stone_type = match (
            payload.sysinfo.is_empty(),
            payload.command_input.is_empty(),
            payload.response.is_empty(),
            payload.file.is_empty(),
        ) {
            (false, true, true, true) => vec![0, 0, 0, 0], // Connection
            (false, false, true, true) => vec![2, 0, 0, 0], // ExecuteCmd
            (false, true, false, true) => vec![3, 0, 0, 0], // Response
            (true, true, true, true) => vec![5, 0, 0, 0],  // Disconnect
            (false, true, true, false) => vec![7, 0, 0, 0], // Upload
            _ => vec![1, 1, 1, 1],                         // HealthCheck
        };

        let stone_size = (payload.sysinfo.len()
            + payload.command_input.len()
            + payload.response.len()
            + payload.file.len()
            + 8)
        .to_le_bytes()[0..4]
            .to_vec();
        let stone_status = 0u32.to_le_bytes().to_vec();

        StructStoneHeader {
            stone_status,
            stone_type,
            stone_size,
        }
    }

    pub fn default() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![0, 0, 0, 0],
            stone_type: vec![0, 0, 0, 0],
            stone_size: vec![12, 0, 0, 0],
        }
    }

    pub fn build(encipher: bool, Type: StoneTransferProtocol, Size: usize) -> StructStoneHeader {
        let mut stone_status: Vec<u8> = match encipher {
            true => vec![0, 0, 0, 1],
            false => vec![0, 0, 0, 0],
        };
        let mut stone_type: Vec<u8> = Type.serialization();
        let mut stone_size: Vec<u8> = Size.to_le_bytes().to_vec();
        stone_size.resize(4, 0);

        StructStoneHeader {
            stone_status,
            stone_type,
            stone_size,
        }
    }
}

impl StructStonePayload {
    pub fn new(command_input: Vec<u8>, response: Vec<u8>, file: Vec<u8>) -> StructStonePayload {
        StructStonePayload {
            sysinfo: StructRawStonePayload::sysinfo().as_bytes().to_vec(),
            command_input,
            response,
            file,
        }
    }

    pub fn from(packet: Vec<u8>) -> StructStonePayload {
        let packet_arr: &[u8] = &packet[..];
        let mut fields: Vec<&[u8]> = packet_arr.split_str("<>").collect();

        while fields.len() < 4 {
            fields.push(b"");
        }

        StructStonePayload {
            sysinfo: fields[0].to_vec(),
            command_input: fields[1].to_vec(),
            response: fields[2].to_vec(),
            file: fields[3].to_vec(),
        }
    }

    pub fn default() -> StructStonePayload {
        StructStonePayload {
            sysinfo: StructRawStonePayload::sysinfo().as_bytes().to_vec(),
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

    pub fn build<T: AsRef<[u8]>>(
        encipher: bool,
        Type: StoneTransferProtocol,
        payload: T,
    ) -> PacketBuilder {
        let output = match Type {
            StoneTransferProtocol::Response | StoneTransferProtocol::ExecuteCmd => {
                StructStonePayload {
                    sysinfo: StructRawStonePayload::sysinfo().as_bytes().to_vec(),
                    command_input: vec![],
                    response: payload.as_ref().to_vec(),
                    file: vec![],
                }
            }
            StoneTransferProtocol::Upload | StoneTransferProtocol::Download => StructStonePayload {
                sysinfo: if Type == StoneTransferProtocol::Download {
                    StructRawStonePayload::sysinfo().as_bytes().to_vec()
                } else {
                    vec![]
                },
                command_input: vec![],
                response: vec![],
                file: payload.as_ref().to_vec(),
            },
            _ => StructStonePayload::default(),
        };

        PacketBuilder {
            encipher,
            Type,
            output,
        }
    }
}

impl StructStone {
    pub fn build(header: StructStoneHeader, payload: StructStonePayload) -> StructStone {
        let mut stone: Vec<u8> = Vec::new();
        stone.extend(&header.stone_status);
        stone.extend(&header.stone_type);
        stone.extend(&header.stone_size);

        println!("{:?}", 0_i32.to_le_bytes().to_vec());

        if header.stone_size == 0_i32.to_le_bytes().to_vec() {
            return StructStone {
                header,
                payload,
                stone,
            };
        }
        stone.extend(&payload.sysinfo);
        stone.extend(b"<>");
        stone.extend(&payload.command_input);
        stone.extend(b"<>");
        stone.extend(&payload.response);
        stone.extend(b"<>");
        stone.extend(&payload.file);
        stone.extend(b"<>");

        StructStone {
            header,
            payload,
            stone,
        }
    }

    pub fn default() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::default())
    }

    pub fn connection() -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::Connection, vec![]).StructStone()
    }

    pub fn disconnect() -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::Disconnect, vec![]).StructStone()
    }

    pub fn response(msg: &str) -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::Response, msg).StructStone()
    }

    pub fn download(file: Vec<u8>) -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::Download, file).StructStone()
    }

    pub fn upload(file: Vec<u8>) -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::Upload, file).StructStone()
    }

    pub fn exploit(output: Vec<u8>) -> StructStone {
        StructStonePayload::build(false, StoneTransferProtocol::ExecuteCmd, output).StructStone()
    }
}
