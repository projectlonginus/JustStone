use crate::structure::{
    enums::{
        EncryptType,
        HandshakeType,
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
};

impl PacketBuilder {
    pub fn packet(&self) -> Packet {
        let mut output = self.output();
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

    pub fn handshake_packet(&self, handshake_type: HandshakeType) -> Result<Packet, ParseError> {
        match SecureHandshakePacket::build(self.raw_packet(), handshake_type, EncryptType::AesGcmSiv) {
            Ok(Packet) => Ok(Packet::from(Packet)),
            Err(Error) => Err(Error)
        }
    }

    pub fn secure_packet(&self, encrypt_type: EncryptType) -> Result<Packet, ParseError> {
        match SecurePacket::build(self.raw_packet(), encrypt_type) {
            Ok(Packet) => Ok(Packet::from(Packet)),
            Err(Error) => Err(Error)
        }
    }
}
