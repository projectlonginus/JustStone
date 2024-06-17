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

impl StoneTransferProtocol {
    pub fn type_check(vec: &Vec<u8>) -> StoneTransferProtocol {
        match vec.as_slice() {
            &[0, 0, 0, 0] => StoneTransferProtocol::Connection,
            &[0, 0, 0, 1] => StoneTransferProtocol::Handshake,
            &[0, 0, 1, 0] => StoneTransferProtocol::HealthCheck,
            &[0, 0, 1, 1] => StoneTransferProtocol::Disconnect,

            &[0, 1, 0, 0] => StoneTransferProtocol::ExecuteCmd,
            &[0, 1, 0, 1] => StoneTransferProtocol::Upload,
            &[0, 1, 1, 0] => StoneTransferProtocol::Download,
            &[0, 1, 1, 1] => StoneTransferProtocol::Response,

            _ => StoneTransferProtocol::Unknown,
        }
    }
}

impl StatusCode {
    pub fn type_check(vec: &Vec<u8>) -> StatusCode {
        match vec.as_slice() {
            &[0, 0, 0, 0] => StatusCode::Normal,
            &[0, 0, 0, 1] => StatusCode::Compressed,
            &[0, 0, 1, 0] => StatusCode::Secured,
            &[0, 0, 1, 1] => StatusCode::SCPacket,
            _ => StatusCode::Modulated
        }
    }
}

impl EncryptType {}

impl HandshakeType {}

impl ProtocolCodec for StoneTransferProtocol {
    fn to_vec(&self) -> Vec<u8>
    {
        match self {
            StoneTransferProtocol::Connection => vec![0, 0, 0, 0],
            StoneTransferProtocol::Handshake => vec![0, 0, 0, 1],
            StoneTransferProtocol::HealthCheck => vec![0, 0, 1, 0],
            StoneTransferProtocol::Disconnect => vec![0, 0, 1, 1],

            StoneTransferProtocol::ExecuteCmd => vec![0, 1, 0, 0],
            StoneTransferProtocol::Upload => vec![0, 1, 0, 1],
            StoneTransferProtocol::Download => vec![0, 1, 1, 0],
            StoneTransferProtocol::Response => vec![0, 1, 1, 1],

            _ => vec![],
        }
    }

    fn to_string(&self) -> String {
        match self {
            StoneTransferProtocol::Connection => "Connection".to_string(),
            StoneTransferProtocol::Handshake => "handshake".to_string(),
            StoneTransferProtocol::HealthCheck => "HealthCheck".to_string(),
            StoneTransferProtocol::Disconnect => "Disconnect".to_string(),

            StoneTransferProtocol::ExecuteCmd => "ExecuteCmd".to_string(),
            StoneTransferProtocol::Upload => "Upload".to_string(),
            StoneTransferProtocol::Download => "Download".to_string(),
            StoneTransferProtocol::Response => "Response".to_string(),

            StoneTransferProtocol::Unknown => "Unknown".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl ProtocolCodec for StatusCode {
    fn to_vec(&self) -> Vec<u8> {
        match self {
            StatusCode::Normal => vec![0, 0, 0, 0],
            StatusCode::Secured => vec![0, 0, 0, 1],
            StatusCode::Compressed => vec![0, 0, 1, 0],
            StatusCode::SCPacket => vec![0, 0, 1, 1],
            _ => vec![]
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
    fn to_vec(&self) -> Vec<u8> {
        match self {
            HandshakeType::RSA => { vec![0, 1] }
            HandshakeType::DiffieHellman => { vec![1, 0] }
            HandshakeType::NoHandshake => { vec![0, 0] }
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
    fn to_vec(&self) -> Vec<u8> {
        match self {
            EncryptType::RSA => { vec![0, 0, 0, 1] }
            EncryptType::AesCbc => { vec![0, 0, 1, 0] }
            EncryptType::AesGcm => { vec![0, 0, 1, 1] }
            EncryptType::AesGcmSiv => { vec![0, 1, 0, 0] }
            _ => { vec![0, 0, 0, 0] }
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
