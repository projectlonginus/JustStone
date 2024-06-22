use crate::structure::utils::{
    structs::define::{SecureHandshakePacket, SecurePacket, StructStone},
    traits::define::Detector,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StoneTransferProtocol {
    Connection,
    Handshake,
    Request,
    Response,
    HealthCheck,
    Disconnect,

    ExecuteCmd,
    Upload,
    Download,

    #[default]
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StatusCode {
    #[default]
    Normal,
    // 압축 x 암호화 x
    Compressed,
    // 압축 o 암호화 x
    Secured,
    // 압축 x 암호화 o
    SCPacket,
    // 압축 o 암호화 o
    Modulated,   // 패킷이 변조되거나 손상됨
}

#[derive(PartialEq, Clone, Default, Debug)]
pub enum HandshakeType {
    RSA,
    DiffieHellman,
    #[default]
    NoHandshake,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum EncryptType {
    RSA,
    AesCbc,
    AesGcm,
    AesGcmSiv,
    #[default]
    NoEncryption,
}

#[derive(Debug, Clone)]
pub enum Packet {
    StructStone(
        StructStone
        // header: crate::structure::structs::define::StructStoneHeader,
        // payload: crate::structure::structs::define::StructStonePayload,
        // stone: Vec<u8>,
    ),
    SecurePacket(
        SecurePacket
        // encrypt_data_block_length: Vec<u8>,
        // encrypted_packet: StructStone,
        // secure_stone: Vec<u8>,
    ),
    SecureHandshakePacket(
        SecureHandshakePacket
        // encrypt_data_block_length: Vec<u8>,
        // handshake_type: Vec<u8>,
        // encrypt_type: Vec<u8>,
        // encrypted_packet: StructStone,
        // secure_stone: Vec<u8>,
    ),
}

#[derive(Debug)]
pub enum PacketError {
    NotStructStone,
    NotSecurePacket,
    NotSecureHandshakePacket,
    UnexpectedError(String),
}

#[derive(Debug)]
pub enum HeaderError {
    StatusIsNot4Bytes,
    TypeIsNot4Bytes,
    SizeIsNot4Bytes,
}

#[derive(Debug)]
pub enum ParseError {
    SizeIsNot4Bytes,
    SizeIsNot2Bytes,
    Unimplemented(String),
}

impl Packet {
    pub fn unwrap<T>(self) -> Result<T, PacketError>
        where
            T: TryFrom<Packet, Error=PacketError>,
    {
        T::try_from(self)
    }

    pub fn from<T>(packet: T) -> Packet
        where
            T: Into<Packet>,
    {
        packet.into()
    }
    pub fn mutable_payload(&mut self) -> &mut dyn Detector {
        return match self {
            Packet::StructStone(packet) => packet as &mut dyn Detector,
            Packet::SecurePacket(packet) => packet as &mut dyn Detector,
            Packet::SecureHandshakePacket(packet) => packet as &mut dyn Detector,
        }
    }

    pub fn payload(&self) -> &dyn Detector {
        return match self {
            Packet::StructStone(packet) => packet,
            Packet::SecurePacket(packet) => packet,
            Packet::SecureHandshakePacket(packet) => packet,
        }
    }
}


