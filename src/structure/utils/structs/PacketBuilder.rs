use std::mem::replace;
use crate::structure::utils::{
    enums::{
        StoneTransferProtocol,
    },
    structs::define::{PacketBuilder, StructStonePayload},
};
use crate::structure::utils::structs::define::EncryptionInfo;

impl PacketBuilder {
    pub fn is_compression(&self) -> &bool {
        &self.compression
    }

    pub fn encryption(&self) -> &EncryptionInfo {
        &self.encryption
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
            encryption: EncryptionInfo::default(),
            protocol: StoneTransferProtocol::default(),
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        compression: bool,
        encryption: EncryptionInfo,
        protocol: StoneTransferProtocol,
        output: StructStonePayload,
    ) -> PacketBuilder {
        PacketBuilder {
            compression,
            encryption,
            protocol,
            output,
        }
    }
}