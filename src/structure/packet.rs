use std::{marker, result};
use crate::structure::{
    utils::{PacketBuilder, StoneTransferProtocol, StructStone, StructStoneHeader,
            StructStonePayload, sysinfo},
    LZ4,
};
use bstr::ByteSlice;
use egui::debug_text::print;
use egui::emath;

impl PacketBuilder {
    pub fn packet(&self) -> StructStone {
        let mut output = self.output();
        StructStone::build(
            StructStoneHeader::build(
                self.encipher(),
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

    fn serialization(&self) -> Vec<u8> {
        let mut header: Vec<u8> = Vec::new();
        header.extend(self.take_stone_status());
        header.extend(self.take_stone_type());
        header.extend(self.take_stone_size());
        header
    }

    pub fn build(
        encipher: &bool,
        protocol: &StoneTransferProtocol,
        size: usize,
    ) -> StructStoneHeader {
        let stone_status: Vec<u8> = match &encipher {
            true => vec![0, 0, 0, 1],
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
        let mut fields: Vec<&[u8]> = packet_arr.split_str("<>").collect();

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

    fn serialization(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = Vec::new();
        payload.extend(self.take_sysinfo());
        payload.extend(b"<>");
        payload.extend(self.take_command_input());
        payload.extend(b"<>");
        payload.extend(self.take_response());
        payload.extend(b"<>");
        payload.extend(self.take_file());
        payload.extend(b"<>");
        let (packet, r) = LZ4::LZ4_compress(&mut payload);
        println!("{:?}", packet);
        packet.to_owned()
    }

    pub fn build<T: AsRef<[u8]>>(
        encipher: bool,
        protocol: StoneTransferProtocol,
        payload: T,
    ) -> PacketBuilder {
        let output = match protocol {
            StoneTransferProtocol::Response | StoneTransferProtocol::ExecuteCmd => StructStonePayload::from(
                sysinfo().as_bytes().to_vec(),
                vec![],
                payload.as_ref().to_vec(),
                vec![],
            ),
            StoneTransferProtocol::Upload | StoneTransferProtocol::Download => StructStonePayload::from(
                if protocol == StoneTransferProtocol::Download {
                    sysinfo().as_bytes().to_vec()
                } else {
                    vec![]
                },
                vec![],
                vec![],
                payload.as_ref().to_vec(),
            ),
            StoneTransferProtocol::Disconnect => Self::new(),
            _ => StructStonePayload::default(),
        };

        PacketBuilder::from(
            encipher,
            protocol,
            output,
        )
    }
}

impl StructStone {
    pub fn build(header: StructStoneHeader, mut payload: StructStonePayload) -> StructStone {
        let mut stone = header.serialization();

        if header.take_stone_size() == &0_i32.to_le_bytes().to_vec() {
            return StructStone::from(
                header,
                payload,
                stone,
            );
        }

        if payload.get_size() != 0 {
            stone.extend(payload.serialization());
        }

        StructStone::from(
            header,
            payload,
            stone,
        )
    }

    pub fn default() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::default())
    }

    pub fn buffer() -> StructStone {
        StructStone::build(StructStoneHeader::default(), StructStonePayload::new())
    }
}
