use crate::structure::utils::enums::{EncryptType, HandshakeType};

#[derive(Clone, Debug, Default)]
pub struct SecureHandshakePacket {
    pub(crate) encryption_flag: [u8; 4],
    pub(crate) encrypt_data_block_length: u32,
    pub(crate) encrypted_packet: Vec<u8>,
    pub(crate) origin_packet: StructStone,
}

#[derive(Clone, Debug, Default)]
pub struct SecurePacket {
    pub(crate) encryption_flag: [u8; 4],
    pub(crate) encrypt_data_block_length: u32,
    pub(crate) encrypted_packet: Vec<u8>,
    pub(crate) origin_packet: StructStone,
}

pub struct StructRawStonePayload {
    pub(crate) sysinfo: String,
    pub(crate) command_input: String,
    pub(crate) response: String,
    pub(crate) file: String,
}

#[derive(Debug, Clone, Default)]
pub struct StructStonePayload {
    pub(crate) sysinfo: Vec<u8>,
    pub(crate) command_input: Vec<u8>,
    pub(crate) response: Vec<u8>,
    pub(crate) file: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct StructStoneHeader {
    pub(crate) stone_status: [u8; 4], // 4바이트
    pub(crate) stone_type:   [u8; 4],   // 4바이트
    pub(crate) stone_size:   u32,   // 4바이트
}

#[derive(Debug, Clone, Default)]
pub struct StructStone {
    pub(crate) header: StructStoneHeader,
    pub(crate) payload: StructStonePayload,
    pub(crate) stone: Vec<u8> ,
}

#[derive(Debug, Clone, Default)]
pub struct EncryptionInfo {
    pub(crate) Activated: bool,
    pub(crate) Type: EncryptType,
    pub(crate) Handshake_Type: HandshakeType
}

#[derive(Debug, Clone, Default)]
pub struct PacketBuilder {
    pub(crate) compression: bool,
    pub(crate) encryption: EncryptionInfo,
    pub(crate) protocol: crate::structure::utils::enums::StoneTransferProtocol,
    pub(crate) output: StructStonePayload,
}