use crate::structure::utils::enums::EncryptType;

#[derive(Clone, Debug, Default)]
pub struct SecureHandshakePacket {
    pub(crate) encrypt_data_block_length: Vec<u8>,
    pub(crate) handshake_type: Vec<u8>,
    pub(crate) encrypt_type: Vec<u8>,
    pub(crate) encrypted_packet: Vec<u8>,
    pub(crate) origin_packet: StructStone,
}

#[derive(Clone, Debug, Default)]
pub struct SecurePacket {
    pub(crate) encrypt_data_block_length: Vec<u8>,
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
    pub(crate) stone_status: Vec<u8>,
    pub(crate) stone_type: Vec<u8>,
    pub(crate) stone_size: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct StructStone {
    pub(crate) header: StructStoneHeader,
    pub(crate) payload: StructStonePayload,
    pub(crate) stone: Vec<u8> ,
}

#[derive(Debug, Clone, Default)]
pub struct PacketBuilder {
    pub(crate) compression: bool,
    pub(crate) encryption: EncryptType,
    pub(crate) protocol: crate::structure::utils::enums::StoneTransferProtocol,
    pub(crate) output: StructStonePayload,
}