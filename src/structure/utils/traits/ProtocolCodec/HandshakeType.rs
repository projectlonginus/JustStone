use crate::structure::utils::enums::HandshakeType;
use crate::structure::utils::traits::define::ProtocolCodec;

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