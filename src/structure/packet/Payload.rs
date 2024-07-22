

use bstr::ByteSlice;
use crate::{
    structure::{
        utils::{
            enums::{
                StoneTransferProtocol,
                EncryptionFlag
            },
            structs::{
                define::{
                    EncryptionInfo,
                    PacketBuilder,
                    StructStonePayload,
                    PACKET_DELIMITER
                }
            },
        }
    },
    utility::{
        LZ4::CompressHandler,
        interface::utils::SystemInterface
    },
};


impl StructStonePayload{

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
        encryption_flag: EncryptionFlag,
        protocol: StoneTransferProtocol,
        payload: T,
    ) -> PacketBuilder {
        let mut vec_payload = payload.as_ref().to_vec();
        let mut sysinfo = SystemInterface::info().as_bytes().to_vec();

        if compression {
            sysinfo.lz4_compress();
            vec_payload.lz4_compress();
        }

        let output = match protocol {
            StoneTransferProtocol::Response | StoneTransferProtocol::ExecuteCmd => {
                StructStonePayload::from(sysinfo, vec![], vec_payload, vec![])
            }
            StoneTransferProtocol::Upload | StoneTransferProtocol::Download => {
                StructStonePayload::from(sysinfo, vec![], vec![], vec_payload)
            }
            StoneTransferProtocol::Disconnect => StructStonePayload::new(),
            _ => StructStonePayload::new(),
        };

        PacketBuilder::from(compression, encryption_flag, protocol, output)
    }
}