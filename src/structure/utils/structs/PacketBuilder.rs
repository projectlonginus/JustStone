use crate::structure::{
    enums::StoneTransferProtocol,
    structs::define::{PacketBuilder, StructStonePayload},
};

impl PacketBuilder {
    pub fn is_compression(&self) -> &bool {
        &self.compression
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
            protocol: StoneTransferProtocol::Unknown,
            output: StructStonePayload::default(),
        }
    }

    pub fn from(
        compression: bool,
        protocol: StoneTransferProtocol,
        output: StructStonePayload,
    ) -> PacketBuilder {
        PacketBuilder {
            compression,
            protocol,
            output,
        }
    }
}