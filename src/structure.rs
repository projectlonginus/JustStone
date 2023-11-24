use std::io::{BufRead, Split};
use bstr::{B, ByteSlice};

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

pub trait Generator {
    fn generator(self) -> StructStone;
}

impl Generator for StructRawStonePayload {

    fn generator(self) -> StructStone{

        let ssp= StructRawStonePayload::to_vec( &self);
        let ssh = StructStoneHeader::from(&ssp);

        println!("보낸거 : {:?} \n보낸거 : {:?}", ssh, ssp);

        StructStone::from(ssh, ssp)

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
            let stone_type = if !payload.sysinfo.is_empty() && payload.command_output.is_empty() && payload.stone_chain.is_empty() {
                [1, 0, 0, 0].to_vec()
            } else if !payload.command_output.is_empty() {
                [2, 0, 0, 0].to_vec()
            } else {
                [3, 0, 0, 0].to_vec()
            };

            let stone_size = (payload.sysinfo.len() + payload.command_input.len() + payload.command_output.len()).to_le_bytes()[0..4].to_vec();
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
            let fields: Vec<&[u8]> = packet_arr.split_str("..").collect();

            println!("{:?}", fields);

            StructStonePayload {
                sysinfo: Vec::from(fields[0]),
                command_input: Vec::from(fields[1]),
                command_output: Vec::from(fields[2]),
                stone_chain: Vec::from(fields[3]),
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



