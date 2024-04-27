use crate::structure::structs::define::{SecureHandshakePacket, SecurePacket, StructStone};

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

#[derive(Debug, Clone, PartialEq)]
pub enum HandshakeType {
    RSA,
    DiffieHellman,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptType {
    RSA,
    AesCbc,
    AesGcm,
    AesGcmSib,
}

#[derive(Debug, Clone)]
pub enum Packet {
    StructStone {
        header: crate::structure::structs::define::StructStoneHeader,
        payload: crate::structure::structs::define::StructStonePayload,
        stone: Vec<u8>,
    },
    SecureHandshakePacket {
        encrypt_data_block_length: Vec<u8>,
        handshake_type: Vec<u8>,
        encrypt_type: Vec<u8>,
        encrypted_packet: Vec<u8>,
    },
    SecurePacket {
        encrypt_data_block_length: Vec<u8>,
        encrypted_packet: Vec<u8>,
    },
}

#[derive(Debug)]
pub enum PacketError {
    NotStructStone,
    NotSecurePacket,
    NotSecureHandshakePacket,
}

impl Packet {
    pub fn to_packet_type<T>(&self) -> Result<T, PacketError>
        where
            T: for<'a> TryFrom<&'a Packet>,
    {
        T::try_from(self)
    }

    pub fn from<T>(packet: T) -> Packet
        where
            T: Into<Packet>,
    {
        packet.into()
    }
}

impl TryFrom<&Packet> for StructStone {
    type Error = PacketError;
    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::StructStone { header, payload, stone } => Ok(StructStone {
                header: header.to_owned(),
                payload: payload.to_owned(),
                stone: stone.to_owned(),
            }),
            _ => Err(PacketError::NotStructStone),
        }
    }
}

impl TryFrom<&Packet> for SecurePacket {
    type Error = PacketError;
    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::SecurePacket { encrypt_data_block_length, encrypted_packet } => Ok(SecurePacket {
                encrypt_data_block_length: encrypt_data_block_length.to_owned(),
                encrypted_packet: encrypted_packet.to_owned(),
            }),
            _ => Err(PacketError::NotSecurePacket),
        }
    }
}

impl TryFrom<&Packet> for SecureHandshakePacket {
    type Error = PacketError;
    fn try_from(packet: &Packet) -> Result<Self, Self::Error> {
        match packet {
            Packet::SecureHandshakePacket { encrypt_data_block_length, handshake_type, encrypt_type, encrypted_packet } => Ok(SecureHandshakePacket {
                encrypt_data_block_length: encrypt_data_block_length.to_owned(),
                handshake_type: handshake_type.to_owned(),
                encrypt_type: encrypt_type.to_owned(),
                encrypted_packet: encrypted_packet.to_owned(),
            }),
            _ => Err(PacketError::NotSecureHandshakePacket),
        }
    }
}

impl From<StructStone> for Packet {
    fn from(packet: StructStone) -> Self {
        Packet::StructStone {
            header: packet.header,
            payload: packet.payload,
            stone: packet.stone,
        }
    }
}

impl From<SecurePacket> for Packet {
    fn from(packet: SecurePacket) -> Self {
        Packet::SecurePacket {
            encrypt_data_block_length: packet.encrypt_data_block_length,
            encrypted_packet: packet.encrypted_packet,
        }
    }
}

impl From<SecureHandshakePacket> for Packet {
    fn from(packet: SecureHandshakePacket) -> Self {
        Packet::SecureHandshakePacket {
            encrypt_data_block_length: packet.encrypt_data_block_length,
            handshake_type: packet.handshake_type,
            encrypt_type: packet.encrypt_type,
            encrypted_packet: packet.encrypted_packet,
        }
    }
}


