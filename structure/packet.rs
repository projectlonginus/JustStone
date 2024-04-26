use bstr::ByteSlice;

use crate::structure::{CompressHandler, Detector, ProtocolCodec, TypeManager};
use crate::structure::utils::{PacketBuilder, StoneTransferProtocol, StructStone, StructStoneHeader,
                              StructStonePayload, sysinfo};

pub const PACKET_DELIMITER: &[u8; 2] = b"\r\n";

impl PacketBuilder {
    pub fn packet(&self) -> StructStone {
        let mut output = self.output();
        StructStone::build(
            StructStoneHeader::build(
                self.is_compression(),
                self.protocol(),
                output.get_size(),
            ),
            output,
        )
    }
}

impl StructStoneHeader {
    pub fn load(packet: Vec<u8>) -> StructStoneHeader {
        if packet[0..4] != vec![0, 0, 0, 0] {
            return StructStoneHeader::default();
        } else {
            StructStoneHeader::from(
                Vec::from(&packet[0..4]),
                Vec::from(&packet[4..8]),
                Vec::from(&packet[8..12]),
            )
        }
    }

    pub fn build(
        compression: &bool,
        protocol: &StoneTransferProtocol,
        size: usize,
    ) -> StructStoneHeader {
        let stone_status: Vec<u8> = match &compression {
            true => vec![1, 0, 0, 0],
            false => vec![0, 0, 0, 0],
        };
        let stone_type: Vec<u8> = protocol.to_vec();
        let mut stone_size: Vec<u8> = size.to_le_bytes().to_vec();
        stone_size.resize(4, 0);

        StructStoneHeader::from(
            stone_status,
            stone_type,
            stone_size,
        )
    }
}

impl StructStonePayload {
    pub fn load(packet: Vec<u8>) -> StructStonePayload {
        let packet_arr: &[u8] = &packet[..];
        let mut fields: Vec<&[u8]> = packet_arr.split_str(PACKET_DELIMITER).collect();

        while fields.len() < 4 {
            fields.push(b"");
        }

        StructStonePayload::from(
            fields[0].to_vec(),
            fields[1].to_vec(),
            fields[2].to_vec(),
            fields[3].to_vec(),
        )
    }

    pub fn build<T: AsRef<[u8]>>(
        compression: bool,
        protocol: StoneTransferProtocol,
        payload: T,
    ) -> PacketBuilder {
        let mut sysinfo = sysinfo().as_bytes().to_vec();
        let mut vec_payload = payload.as_ref().to_vec();
        if compression {
            sysinfo.lz4_compress();
            vec_payload.lz4_compress();
        }
        let output = match protocol {
            StoneTransferProtocol::Response | StoneTransferProtocol::ExecuteCmd => StructStonePayload::from(
                sysinfo,
                vec![],
                vec_payload,
                vec![],
            ),
            StoneTransferProtocol::Upload | StoneTransferProtocol::Download => StructStonePayload::from(
                if protocol == StoneTransferProtocol::Download {
                    sysinfo
                } else {
                    vec![]
                },
                vec![],
                vec![],
                vec_payload,
            ),
            StoneTransferProtocol::Disconnect => Self::new(),
            _ => StructStonePayload::default(),
        };
        PacketBuilder::from(
            compression,
            protocol,
            output,
        )
    }
}

impl StructStone {
    pub fn build(header: StructStoneHeader, mut payload: StructStonePayload) -> StructStone {
        let mut stone: Vec<u8> = header.to_vec();
        if header.take_stone_size().as_slice() == &0_i32.to_le_bytes() {
            return StructStone::from(header, payload, stone);
        }
        if !payload.is_empty() {
            stone.extend(payload.to_vec());
        }
        StructStone::from(header, payload, stone)
    }

    pub fn default() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::default())
    }

    pub fn buffer() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::new())
    }
}
