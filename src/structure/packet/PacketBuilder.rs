use crate::{
    structure::utils::{
        enums::{
            EncryptType,
            HandshakeType,
            Packet,
            ParseError
        },
        structs::define::{
            PacketBuilder,
            SecureHandshakePacket,
            SecurePacket,
            StructStone,
            StructStoneHeader,
        },
        traits::define::Detector
    }
};

impl PacketBuilder {
    pub fn packet(&self) -> Packet {
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

    pub fn raw_packet(&self) -> StructStone {
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

    pub fn handshake_packet(&self, handshake_type: &HandshakeType) -> Result<Packet, ParseError> {
        match SecureHandshakePacket::build(self.raw_packet(), handshake_type, &self.encryption) {
            Ok(packet) => Ok(Packet::from(packet)),
            Err(error) => Err(error)
        }
    }

    pub fn secure_packet(&self) -> Result<Packet, ParseError> {
        match SecurePacket::build(self.raw_packet(), &self.encryption) {
            Ok(packet) => Ok(Packet::from(packet)),
            Err(error) => Err(error)
        }
    }

    pub fn load_builder(packet: &StructStone) -> PacketBuilder {
        let encryption = match packet.is_encrypted() {
            true => EncryptType::AesGcmSiv,
            false => EncryptType::NotEncryption
        };
        PacketBuilder::from(packet.is_compression(), encryption, packet.get_type(), packet.get_payload())
    }
}
