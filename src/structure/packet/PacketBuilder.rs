use crate::structure::utils::{
    enums::{
        Packet,
        ParseError,
    },
    structs::define::{
        PacketBuilder,
        SecureHandshakePacket,
        SecurePacket,
        StructStone,
        StructStoneHeader,
    },
    traits::define::Detector,
};

impl PacketBuilder {
    pub fn packet(&mut self) -> Packet {
        let output = self.output();
        Packet::from(
            StructStone::build(
                StructStoneHeader::build(
                    self.is_compression(),
                    self.protocol(),
                    output.get_size(),
                ),
                output,
            )
        )
    }

    pub fn raw_packet(&mut self) -> StructStone {
        let output = self.output();
        StructStone::build(
            StructStoneHeader::build(
                self.is_compression(),
                self.protocol(),
                output.get_size(),
            ),
            output,
        )
    }

    pub fn handshake_packet(&mut self) -> Result<Packet, ParseError> {
        match SecureHandshakePacket::build(self.raw_packet(), self.encryption()) {
            Ok(packet) => Ok(Packet::from(packet)),
            Err(error) => Err(error)
        }
    }

    pub fn secure_packet(&mut self) -> Result<Packet, ParseError> {
        match SecurePacket::build(self.raw_packet(), &self.encryption) {
            Ok(packet) => Ok(Packet::from(packet)),
            Err(error) => Err(error)
        }
    }

    pub fn load_builder(packet: &mut Packet) -> PacketBuilder {
        PacketBuilder::from(packet.is_compression(), packet.get_encryption(), packet.get_type(), packet.get_payload())
    }
}
