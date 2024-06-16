use crate::structure::{
    enums::{
        EncryptType,
        StoneTransferProtocol,
    },
    structs::define::{PacketBuilder, StructStonePayload},
};

impl PacketBuilder {
    pub fn is_compression(&self) -> &bool {
        &self.compression
    }

    pub fn encryption(&self) -> &EncryptType {
        &self.encryption
    }

    pub fn protocol(&self) -> &StoneTransferProtocol {
        &self.protocol
    }

    pub fn output(&self) -> StructStonePayload {
        self.output.clone()
    }

    pub fn default() -> PacketBuilder {
        PacketBuilder {
            compression: false,
            encryption: EncryptType::AesGcmSiv,
            protocol: StoneTransferProtocol::Unknown,
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        compression: bool,
        encryption: EncryptType,
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