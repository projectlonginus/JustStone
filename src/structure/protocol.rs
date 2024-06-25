use crate::{
    structure::{
        utils::{
            enums::{
                EncryptType,
                HandshakeType,
                StatusCode,
                StoneTransferProtocol
            },
            traits::define::ProtocolCodec
        }
    }
};
use crate::structure::utils::enums::EncryptionFlag;

impl ProtocolCodec for StoneTransferProtocol {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0, 0, 0, 0] => StoneTransferProtocol::Connection,
            [0, 0, 0, 1] => StoneTransferProtocol::Handshake,
            [0, 0, 1, 0] => StoneTransferProtocol::HealthCheck,
            [0, 0, 1, 1] => StoneTransferProtocol::Disconnect,

            [0, 1, 0, 0] => StoneTransferProtocol::ExecuteCmd,
            [0, 1, 0, 1] => StoneTransferProtocol::Upload,
            [0, 1, 1, 0] => StoneTransferProtocol::Download,
            [0, 1, 1, 1] => StoneTransferProtocol::Response,

            _ => StoneTransferProtocol::Unknown,
        }
    }

    fn to_bytes(&self) -> [u8; 4]
    {
        match self {
            StoneTransferProtocol::Connection   => [0, 0, 0, 0],
            StoneTransferProtocol::Handshake    => [0, 0, 0, 1],
            StoneTransferProtocol::HealthCheck  => [0, 0, 1, 0],
            StoneTransferProtocol::Disconnect   => [0, 0, 1, 1],

            StoneTransferProtocol::ExecuteCmd   => [0, 1, 0, 0],
            StoneTransferProtocol::Upload       => [0, 1, 0, 1],
            StoneTransferProtocol::Download     => [0, 1, 1, 0],
            StoneTransferProtocol::Response     => [0, 1, 1, 1],

            _ => [1,0,0,0],
        }
    }

    fn to_string(&self) -> String {
        match self {
            StoneTransferProtocol::Connection   => "Connection".to_string(),
            StoneTransferProtocol::Handshake    => "handshake".to_string(),
            StoneTransferProtocol::HealthCheck  => "HealthCheck".to_string(),
            StoneTransferProtocol::Disconnect   => "Disconnect".to_string(),

            StoneTransferProtocol::ExecuteCmd   => "ExecuteCmd".to_string(),
            StoneTransferProtocol::Upload       => "Upload".to_string(),
            StoneTransferProtocol::Download     => "Download".to_string(),
            StoneTransferProtocol::Response     => "Response".to_string(),

            StoneTransferProtocol::Unknown      => "Unknown".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl ProtocolCodec for StatusCode {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0, 0, 0, 0] => StatusCode::Normal,     // 압축 x 서명 x
            [0, 0, 0, 1] => StatusCode::Compressed, // 압축 o 서명 x
            [0, 0, 1, 0] => StatusCode::Secured,    // 압축 x 서명 o
            [0, 0, 1, 1] => StatusCode::SCPacket,   // 압축 o 서명 o
            _ => StatusCode::Modulated // 패킷이 변조되거나 손상됨
        }
    }

    fn to_bytes(&self) -> [u8; 4] {
        match self {
            StatusCode::Normal =>     [0, 0, 0, 0],
            StatusCode::Secured =>    [0, 0, 0, 1],
            StatusCode::Compressed => [0, 0, 1, 0],
            StatusCode::SCPacket =>   [0, 0, 1, 1],
            _ =>  [0,1,0,0]
        }
    }

    fn to_string(&self) -> String {
        match self {
            StatusCode::Normal => "Normal".to_string(),
            StatusCode::Secured => "Secured".to_string(),
            StatusCode::Compressed => "Compressed".to_string(),
            StatusCode::SCPacket => "SCPacket".to_string(),
            _ => "".to_string()
        }
    }
}

impl ProtocolCodec for HandshakeType {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0,0,0,1] => HandshakeType::RSA,
            [0,0,1,0] => HandshakeType::DiffieHellman,
            _ => HandshakeType::NoHandshake
        }
    }

    fn to_bytes(&self) -> [u8; 4] {
        match self {
            HandshakeType::NoHandshake      => { [0,0,0,0] }
            HandshakeType::RSA              => { [0,0,0,1] }
            HandshakeType::DiffieHellman    => { [0,0,1,0] }
        }
    }

    fn to_string(&self) -> String {
        match self {
            HandshakeType::RSA => { "RSA".to_string() }
            HandshakeType::DiffieHellman => { "DiffieHellman".to_string() }
            HandshakeType::NoHandshake => { "NoHandshake".to_string() }
        }
    }
}


impl ProtocolCodec for EncryptType {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec[..] {
            [0, 0, 0, 1] => EncryptType::RSA,
            [0, 0, 1, 0] => EncryptType::AesCbc,
            [0, 0, 1, 1] => EncryptType::AesGcm,
            [0, 1, 0, 0] => EncryptType::AesGcmSiv,
            _ => { EncryptType::NoEncryption }
        }
    }

    fn to_bytes(&self) -> [u8; 4] {
        match self {
            EncryptType::RSA        => { [0, 0, 0, 1] }
            EncryptType::AesCbc     => { [0, 0, 1, 0] }
            EncryptType::AesGcm     => { [0, 0, 1, 1] }
            EncryptType::AesGcmSiv  => { [0, 1, 0, 0] }
            _ => { [0, 0, 0, 0] }
        }
    }

    fn to_string(&self) -> String {
        match self {
            EncryptType::RSA => { "RSA".to_string() }
            EncryptType::AesCbc => { "AesCbc".to_string() }
            EncryptType::AesGcm => { "AecCbc".to_string() }
            EncryptType::AesGcmSiv => { "AesGcmSiv".to_string() }
            _ => { "NotEncryption".to_string() }
        }
    }
}

impl ProtocolCodec for EncryptionFlag {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0, 1, 0, 1] => EncryptionFlag::RAGS,
            [0, 1, 1, 0] => EncryptionFlag::RAG,
            [0, 1, 1, 1] => EncryptionFlag::RAC,
            [1, 0, 0, 0] => EncryptionFlag::DHAGS,
            [1, 0, 0, 1] => EncryptionFlag::DHAG,
            [1, 0, 1, 0] => EncryptionFlag::DHAC,
            _ => { EncryptionFlag::Unknown }
        }
    }

    fn to_bytes(&self) -> [u8; 4] {
        match self {
            EncryptionFlag::RAGS    => [0, 1, 0, 1],
            EncryptionFlag::RAG     => [0, 1, 1, 0],
            EncryptionFlag::RAC     => [0, 1, 1, 1],
            EncryptionFlag::DHAGS   => [1, 0, 0, 0],
            EncryptionFlag::DHAG    => [1, 0, 0, 1],
            EncryptionFlag::DHAC    => [1, 0, 1, 0],
            _ => [1, 0, 1, 1]
        }
    }

    fn to_string(&self) -> String {
        match self {
            EncryptionFlag::RAGS    => "RSA-AesGcmSiv".to_string(),
            EncryptionFlag::RAG     => "RSA-AesGcm".to_string(),
            EncryptionFlag::RAC     => "RSA-AesCbc".to_string(),
            EncryptionFlag::DHAGS   => "Diffie_Hellman-AesGcmSiv".to_string(),
            EncryptionFlag::DHAG    => "Diffie_Hellman-AesGcm".to_string(),
            EncryptionFlag::DHAC    => "Diffie_Hellman-AesCbc".to_string(),
            _ => "Unknown".to_string()
        }
    }
}