#![allow(dead_code)]

use crate::stprotocol::utils::{HandleSessions, NormalSessionLayer, SecureSessionLayer};
use crate::structure::utils::{
    structs::define::{
        EncryptionInfo,
        SecureHandshakePacket,
        SecurePacket,
        StructStone
    },
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
    Unknown, // 알수없는 패킷 유형
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum StatusCode {
    #[default]
    Normal,
    // 압축 x 서몀 x
    Compressed,
    // 압축 o 서명 x
    Secured,
    // 압축 x 서명 o
    SCPacket,
    // 압축 o 서명 o
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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum EncryptionFlag { // 8바이트 길이 암호화 플레그
    RAC, // 핸드셰이크: RSA, 패킷 암호화 알고리즘: AesCbc
    RAG, // 핸드셰이크: RSA, 패킷 암호화 알고리즘: AesGcm
    RAGS, // 핸드셰이크: RSA, 패킷 암호화 알고리즘: AesGcmSiv
    DHAC, // 핸드셰이크: Diffie-Hellman, 패킷 암호화 알고리즘: AesCbc
    DHAG, // 핸드셰이크: Diffie-Hellman, 패킷 암호화 알고리즘: AesGcm
    DHAGS,// 핸드셰이크: Diffie-Hellman, 패킷 암호화 알고리즘: AesGcmSiv
    Unknown, // 알수없는 암호화 유형
    #[default]
    NoEncryption
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
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
    #[default]
    Default
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum Sessions {
    NormalSession(dyn NormalSessionLayer),
    SecureSession(dyn SecureSessionLayer),
    #[default]
    Default
}

pub enum SessionParsingError {
    NormalSessionParsingError(String),
    SecureSessionParsingError(String),
    UnexpectedDefaultSession,
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

impl Sessions {
    pub fn unwrap<T>(self) -> Result<T, SessionParsingError>
    where
        T: TryFrom<Sessions, Error=SessionParsingError>,
    {
        T::try_from(self)
    }

    pub fn from<T>(packet: T) -> Packet
    where
        T: Into<Packet>,
    {
        packet.into()
    }
    pub fn mutable_session(&mut self) -> Result<&mut dyn HandleSessions, SessionParsingError>  {
        return match self {
            Sessions::NormalSession(session) => Ok(session as &mut dyn HandleSessions),
            Sessions::SecureSession(session) => Ok(session as &mut dyn HandleSessions),
            Sessions::Default => Err(SessionParsingError::UnexpectedDefaultSession)
        }
    }

    pub fn session(&self) -> Result<&dyn HandleSessions, SessionParsingError> {
        return match self {
            Sessions::NormalSession(session) => Ok(session),
            Sessions::SecureSession(session) => Ok(session),
            Sessions::Default => Err(SessionParsingError::UnexpectedDefaultSession)
        }
    }
    pub fn default() -> Self {
        Self::default()
    }
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
    pub fn mutable_payload(&mut self) -> Result<&mut dyn Detector,StructStone>  {
        return match self {
            Packet::StructStone(packet) => Ok(packet as &mut dyn Detector),
            Packet::SecurePacket(packet) => Ok(packet as &mut dyn Detector),
            Packet::SecureHandshakePacket(packet) => Ok(packet as &mut dyn Detector),
            Packet::Default => Err(StructStone::default())
        }
    }

    pub fn payload(&self) -> Result<&dyn Detector,StructStone> {
        return match self {
            Packet::StructStone(packet) => Ok(packet),
            Packet::SecurePacket(packet) => Ok(packet),
            Packet::SecureHandshakePacket(packet) => Ok(packet),
            Packet::Default => Err(StructStone::default())
        }
    }
    pub fn default() -> Self {
        Packet::Default
    }
}

impl EncryptionFlag {
    pub fn get_types(&self) -> EncryptionInfo {
        let element = match self {
            EncryptionFlag::RAGS    => (true, EncryptType::AesGcmSiv, HandshakeType::RSA),
            EncryptionFlag::RAG     => (true, EncryptType::AesGcm,    HandshakeType::RSA),
            EncryptionFlag::RAC     => (true, EncryptType::AesCbc,    HandshakeType::RSA),
            EncryptionFlag::DHAGS   => (true, EncryptType::AesGcmSiv, HandshakeType::DiffieHellman),
            EncryptionFlag::DHAG    => (true, EncryptType::AesGcm,    HandshakeType::DiffieHellman),
            EncryptionFlag::DHAC    => (true, EncryptType::AesCbc,    HandshakeType::DiffieHellman),
            _ => (false, EncryptType::NoEncryption, HandshakeType::NoHandshake),
        };
        EncryptionInfo {
            Activated:      element.0,
            Type:           element.1,
            Handshake_Type: element.2,
        }
    }
    pub fn get_encryption_type(&self) -> EncryptType {
        self.get_types().Type
    }
    pub fn get_handshake_type(&self) -> HandshakeType {
        self.get_types().Handshake_Type
    }
}
