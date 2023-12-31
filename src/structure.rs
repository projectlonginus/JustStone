use bstr::ByteSlice;
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
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructStonePayload {
    sysinfo: Vec<u8>,
    command_input: Vec<u8>,
    response: Vec<u8>,
    file: Vec<u8>, // pub stone_chain: StoneChain,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructStoneHeader {
    stone_status: Vec<u8>,
    stone_type: Vec<u8>,
    stone_size: Vec<u8>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StructStone {
    pub header: StructStoneHeader,
    pub payload: StructStonePayload,
    pub stone: Vec<u8>,
}

#[derive(Debug)]
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

pub trait Generator {
    fn to_stone(self) -> StructStone;
}

impl Generator for StructRawStonePayload {
    fn to_stone(self) -> StructStone {
        let ssp = StructRawStonePayload::to_vec(&self);
        let ssh = StructStoneHeader::from(&ssp);

        StructStone::from(ssh, ssp)
    }
}

impl Generator for StructStonePayload {
    fn to_stone(self) -> StructStone {
        let ssh = StructStoneHeader::from(&self);

        StructStone::from(ssh, self)
    }
}

pub trait Detector {
    fn header_type(&self) -> StoneTransferProtocol;
    fn payload_size(&self) -> usize;
    fn get_command(&self) -> Vec<u8>;
    fn get_file(&self) -> Vec<u8>;
}

impl Detector for StructStone {
    fn header_type(&self) -> StoneTransferProtocol {
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
    fn payload_size(&self) -> usize {
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
    pub fn to_vec(&self) -> StructStonePayload {
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
    pub fn load(packet: Vec<u8>) -> StructStoneHeader {
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

    // pub fn upload(payload: &StructStonePayload) -> StructStoneHeader {
    //     StructStoneHeader {
    //         stone_status: vec![0,0,0,0],
    //         stone_type: vec![7,0,0,0],
    //         stone_size: Vec::from(payload.file.len().to_le_bytes()),
    //     }
    // }
    //
    // pub fn download(payload: &StructStonePayload) -> StructStoneHeader {
    //     StructStoneHeader {
    //         stone_status: vec![0,0,0,0],
    //         stone_type: vec![8,0,0,0],
    //         stone_size: Vec::from(payload.file.len().to_le_bytes()),
    //     }
    // }

    pub fn default() -> StructStoneHeader {
        StructStoneHeader {
            stone_status: vec![0, 0, 0, 0],
            stone_type: vec![0, 0, 0, 0],
            stone_size: vec![12, 0, 0, 0],
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
            sysinfo: vec![],
            command_input: vec![],
            response: vec![],
            file: vec![],
        }
    }
}

impl StructStone {
    pub fn from(header: StructStoneHeader, payload: StructStonePayload) -> StructStone {
        let mut stone: Vec<u8> = Vec::new();
        stone.extend(&header.stone_status);
        stone.extend(&header.stone_type);
        stone.extend(&header.stone_size);
        stone.extend(&payload.sysinfo);
        stone.extend("<>".as_bytes().to_vec());
        stone.extend(&payload.command_input);
        stone.extend("<>".as_bytes().to_vec());
        stone.extend(&payload.response);
        stone.extend("<>".as_bytes().to_vec());
        stone.extend(&payload.file);
        stone.extend("<>".as_bytes().to_vec());

        StructStone {
            header,
            payload,
            stone,
        }
    }

    pub fn default() -> StructStone {
        StructStone {
            header: StructStoneHeader::default(),
            payload: StructStonePayload::default(),
            stone: StructStone::from(StructStoneHeader::default(), StructStonePayload::default())
                .stone,
        }
    }

    pub fn disconnect() -> StructStone {
        let header = StructStoneHeader {
            stone_status: vec![0, 0, 0, 0],
            stone_type: vec![5, 0, 0, 0],
            stone_size: vec![0, 0, 0, 0],
        };

        StructStone {
            header: header.clone(),
            payload: StructStonePayload::default(),
            stone: StructStone::from(header, StructStonePayload::default()).stone,
        }
    }

    pub fn response(msg: &str) -> StructStone {
        let payload = StructStonePayload {
            sysinfo: vec![],
            command_input: vec![],
            response: msg.as_bytes().to_vec(),
            file: vec![],
        };

        let header = StructStoneHeader::from(&payload);

        StructStone {
            header: header.clone(),
            payload: payload.clone(),
            stone: StructStone::from(header, payload).stone,
        }
    }

    pub fn download(file: Vec<u8>) -> StructStone {
        let payload = StructStonePayload {
            sysinfo: vec![],
            command_input: vec![],
            response: vec![],
            file,
        };

        let header = StructStoneHeader::from(&payload);

        StructStone {
            header: header.clone(),
            payload: payload.clone(),
            stone: StructStone::from(header, payload).stone,
        }
    }

    pub fn upload(file: Vec<u8>) -> StructStone {
        let payload = StructStonePayload {
            sysinfo: vec![],
            command_input: vec![],
            response: vec![],
            file,
        };

        let header = StructStoneHeader::from(&payload);

        StructStone {
            header: header.clone(),
            payload: payload.clone(),
            stone: StructStone::from(header, payload).stone,
        }
    }

    pub fn exploit(output: Vec<u8>) -> StructStone {
        let payload = StructStonePayload {
            sysinfo: vec![],
            command_input: vec![],
            response: output,
            file: vec![],
        };

        let header = StructStoneHeader::from(&payload);

        StructStone {
            header: header.clone(),
            payload: payload.clone(),
            stone: StructStone::from(header, payload).stone,
        }
    }
}
