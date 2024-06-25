use crate::structure::utils::enums::EncryptionFlag;
use crate::structure::utils::traits::define::ProtocolCodec;

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