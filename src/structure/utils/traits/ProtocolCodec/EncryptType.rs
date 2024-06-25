use crate::structure::utils::enums::EncryptType;
use crate::structure::utils::traits::define::ProtocolCodec;

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
            EncryptType::RSA        => [0, 0, 0, 1],
            EncryptType::AesCbc     => [0, 0, 1, 0],
            EncryptType::AesGcm     => [0, 0, 1, 1],
            EncryptType::AesGcmSiv  => [0, 1, 0, 0],
            _ => [0, 0, 0, 0]
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