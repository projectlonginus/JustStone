use std::io::{BufRead, Split};
use bstr::{B, ByteSlice};
use crate::exploit::Exploits;

pub struct  StoneChain {
    pub previous_stone_hash: Vec<u8>,
    pub stone_hash:          Vec<u8>,
    pub stonetree_hash:      Vec<u8>,
    pub timestamp:           Vec<u8>,
    pub transaction_list:    Vec<u8>
}
pub struct StructRawStonePayload {
    pub sysinfo:        String,
    pub command_input:  String,
    pub command_output: String,
    pub stone_chain:    String,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StructStonePayload {
    pub sysinfo:        Vec<u8>,
    pub command_input:  Vec<u8>,
    pub command_output: Vec<u8>,
    pub stone_chain:    Vec<u8>
    // pub stone_chain: StoneChain,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StructStoneHeader {
    pub stone_status: Vec<u8>,
    pub stone_type:   Vec<u8>,
    pub stone_size:   Vec<u8>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct StructStone {
    pub header:  StructStoneHeader,
    pub payload: StructStonePayload,
    pub stone:  Vec<u8>
}

#[derive(Debug)]
pub enum StoneTransferProtocol {
    Connection,
    Handshake,
    HealthCheck,
    Request,
    Response,
    Disconnect,
    Unknown
}

pub trait Generator {
    fn to_stone(self) -> StructStone;
}

impl Generator for StructRawStonePayload {

    fn to_stone(self) -> StructStone{

        let ssp= StructRawStonePayload::to_vec( &self);
        let ssh = StructStoneHeader::from(&ssp);

        StructStone::from(ssh, ssp)

    }
}

impl Generator for StructStonePayload {

    fn to_stone(self) -> StructStone{
        let ssh = StructStoneHeader::from(&self);

        StructStone::from(ssh, self)

    }
}

pub trait Detector {
    fn header_type(&self) -> StoneTransferProtocol;
}

impl Detector for StructStone {
    fn header_type(&self) -> StoneTransferProtocol {
        match &self.header.stone_type.as_slice() {
            &[ 0,0,0,0 ] => StoneTransferProtocol::Connection,
            &[ 1,0,0,0 ] => StoneTransferProtocol::Response,
            &[ 2,0,0,0 ] => StoneTransferProtocol::Request,
            &[ 3,0,0,0 ]=> StoneTransferProtocol::HealthCheck,
            &[ 4,0,0,0 ]=> StoneTransferProtocol::Disconnect,
            _ => StoneTransferProtocol::Unknown
        }
    }
}

impl StructRawStonePayload {
    pub fn to_vec(&self) ->StructStonePayload {
        let sysinfo        = self.sysinfo.as_bytes().to_vec();
        let command_input  = self.command_input.as_bytes().to_vec();
        let command_output = self.command_output.as_bytes().to_vec();

        StructStonePayload {
            sysinfo,
            command_input,
            command_output,
            stone_chain: vec![]
            // stone_chain: StoneChain {
            //     previous_stone_hash : vec![],
            //     stone_hash          : vec![],
            //     stonetree_hash      : vec![],
            //     timestamp           : vec![],
            //     transaction_list    : vec![]
            // }
        }
    }
}

impl StructStoneHeader {
        pub fn load(packet: Vec<u8>) -> StructStoneHeader {
            StructStoneHeader {
                stone_status: Vec::from(&packet[0..4]),
                stone_type: Vec::from(&packet[4..8]),
                stone_size: Vec::from(&packet[8..12]),
            }
        }

        pub fn from(payload: &StructStonePayload) -> StructStoneHeader {
            let stone_type = match (
                payload.sysinfo.is_empty(),
                payload.command_input.is_empty(),
                payload.command_output.is_empty(),
            ) {
                (false, true, true) =>  vec![0, 0, 0, 0], // Connection
                (false, true, false) => vec![1, 0, 0, 0], // Response
                (false, false, true) => vec![2, 0, 0, 0], // Request
                (true, true, true) =>   vec![4, 0, 0, 0], // Request
                _ => vec![3, 0, 0, 0],                    // HealthCheck
            };



            let stone_size = (payload.sysinfo.len() + payload.command_input.len() + payload.command_output.len() + 8).to_le_bytes()[0..4].to_vec();
            let stone_status = 0u32.to_le_bytes().to_vec();

            StructStoneHeader {
                stone_status,
                stone_type ,
                stone_size,
            }
        }

    pub fn default() -> StructStoneHeader{
        StructStoneHeader {
            stone_status: vec![0,0,0,0],
            stone_type: vec![0,0,0,0],
            stone_size: vec![12,0,0,0],
        }
    }
}

    impl  StructStonePayload {
        pub fn from(packet: Vec<u8>) -> StructStonePayload {
            let packet_arr:&[u8] = &packet[..];
            let mut fields: Vec<&[u8]> = packet_arr.split_str("..").collect();

            while fields.len() < 4 {
                fields.push(b"");
            }

            StructStonePayload {
                sysinfo: fields[0].to_vec(),
                command_input: fields[1].to_vec(),
                command_output: fields[2].to_vec(),
                stone_chain: fields[3].to_vec(),
            }
        }

        pub fn from_ex(exploit: Exploits) -> StructStonePayload {

            StructStonePayload {
                sysinfo: exploit.sys_info,
                command_input: exploit.exploit_input,
                command_output: exploit.exploit_output,
                stone_chain: vec![],
            }
        }

        pub fn default() -> StructStonePayload {
            StructStonePayload {
                sysinfo: vec![],
                command_input: vec![],
                command_output: vec![],
                stone_chain: vec![],
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
        stone.extend(&payload.command_input);
        stone.extend(&payload.command_output);
        stone.extend(&payload.stone_chain);


        StructStone {
            header, payload, stone
        }
    }

    pub  fn default() -> StructStone{
        StructStone {
            header: StructStoneHeader::default(),
            payload: StructStonePayload::default(),
            stone: Vec::new(),
        }
    }
}



