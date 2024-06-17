use crate::structure::structs::define::{SecureHandshakePacket, SecurePacket, StructStone};
use crate::structure::traits::define::Detector;

#[derive(Debug, Clone, PartialEq)]
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

    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
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

#[derive(PartialEq)]
pub enum HandshakeType {
    RSA,
    DiffieHellman,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptType {
    RSA,
    AesCbc,
    AesGcm,
    AesGcmSiv,
    NotEncryption,
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
    pub fn payload(&self) -> Option<&dyn Detector> {
        match self {
            Packet::StructStone(payload) => Some(payload),
            Packet::SecurePacket(payload) => Some(payload),
            Packet::SecureHandshakePacket(payload) => Some(payload),
        }
    }
}


