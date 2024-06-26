use bstr::ByteSlice;
use sysinfo::System;

use crate::{
    structure::utils::{
        enums::StoneTransferProtocol,
        structs::define::{
            EncryptionInfo,
            PacketBuilder,
            StructStonePayload
        }
    },
    utility::LZ4::CompressHandler,
};

pub const PACKET_DELIMITER: &[u8; 2] = b"\r\n";

impl StructStonePayload {
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
        encryption: EncryptionInfo,
        protocol: StoneTransferProtocol,
        payload: T,
    ) -> PacketBuilder {
        let mut vec_payload = payload.as_ref().to_vec();
        let mut sysinfo = vec![];

        if compression {
            sysinfo.lz4_compress();
            vec_payload.lz4_compress();
        }
        
        let output = match protocol {
            StoneTransferProtocol::Handshake => {
                StructStonePayload::from(vec![], vec![], vec![], vec_payload)
            }
            StoneTransferProtocol::Response | StoneTransferProtocol::ExecuteCmd => {
                StructStonePayload::from(sysinfo, vec![], vec_payload, vec![])
            }
            StoneTransferProtocol::Upload | StoneTransferProtocol::Download => {
                let (sys, pay) = if protocol == StoneTransferProtocol::Download {
                    (sysinfo, vec![])
                } else {
                    (vec![], vec_payload)
                };
                StructStonePayload::from(sys, vec![], vec![], pay)
            }
            StoneTransferProtocol::Disconnect => StructStonePayload::default(),
            _ => StructStonePayload::default(),
        };

        println!("build 프로토콜: {:?}", protocol);
        println!("build 페이로드: {:?}", output);

        PacketBuilder::from(compression, encryption, protocol, output)
    }

    pub fn default() -> StructStonePayload {
        StructStonePayload {
            sysinfo: Self::sysinfo().as_bytes().to_vec(),
            command_input: vec![],
            response: vec![],
            file: vec![],
        }
    }
}