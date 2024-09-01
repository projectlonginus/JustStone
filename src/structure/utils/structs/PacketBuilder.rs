#![allow(dead_code)]

use std::mem::replace;
use crate::{
    structure::{
        utils::{
            enums::{
                StoneTransferProtocol,
                EncryptionFlag
            },
            structs::{
                define::{
                    PacketBuilder,
                    StructStonePayload,
                }
            },
        }
    }
};

impl PacketBuilder {
    pub fn is_compression(&self) -> &bool {
        &self.compression
    }

    pub fn encryption(&self) -> &EncryptionFlag {
        &self.encryption_flag
    }

    pub fn protocol(&self) -> &StoneTransferProtocol {
        &self.protocol
    }

    pub fn output(&mut self) -> StructStonePayload {
        replace(&mut self.output, Default::default())
    }

    pub fn default() -> PacketBuilder {
        PacketBuilder {
            compression: false,
            encryption_flag: EncryptionFlag::default(),
            protocol: StoneTransferProtocol::default(),
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        compression: bool,
        encryption_flag: EncryptionFlag,
        protocol: StoneTransferProtocol,
        output: StructStonePayload,
    ) -> PacketBuilder {
        PacketBuilder {
            compression,
            encryption_flag,
            protocol,
            output,
        }
    }
}