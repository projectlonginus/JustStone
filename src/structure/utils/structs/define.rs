use crate::structure::utils::enums::{EncryptionFlag, EncryptType, HandshakeType, StoneTransferProtocol};

pub const PACKET_DELIMITER: &[u8; 2] = b"\r\n";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SecureHandshakePacket {
    pub(crate) encryption_flag: [u8; 4],
    pub(crate) encrypt_data_block_length: u32,
    pub(crate) encrypted_packet: Vec<u8>,
    pub(crate) origin_packet: StructStone,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SecurePacket {
    // pub(crate) encryption_flag: [u8; 4],
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

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct StructStonePayload {
    pub(crate) sysinfo: Vec<u8>,
    pub(crate) command_input: Vec<u8>,
    pub(crate) response: Vec<u8>,
    pub(crate) file: Vec<u8>,
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct StructStoneHeader {
    pub(crate) stone_status: [u8; 4], // 4바이트
    pub(crate) stone_type:   [u8; 4],   // 4바이트
    pub(crate) stone_size:   u32,   // 4바이트
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct StructStone {
    pub(crate) header: StructStoneHeader,
    pub(crate) payload: StructStonePayload,
    pub(crate) stone: Vec<u8> ,
}

#[derive(Debug, Clone, Default)] // enum 타입과 역/작렬화가 가능하도록 구현 생각중
pub struct EncryptionInfo {
    pub(crate) Activated: bool, // 암호화가 되었는가?
    pub(crate) Type: EncryptType, // 어떤 방식으로 암호화 할것인가?
    pub(crate) Handshake_Type: HandshakeType // 어떤 방식으로 핸드셰이킹 할것인가?
}

#[derive(Debug, Clone, Default)]
pub struct PacketBuilder {
    pub(crate) compression: bool,
    pub(crate) encryption_flag: EncryptionFlag,
    pub(crate) protocol: StoneTransferProtocol,
    pub(crate) output: StructStonePayload,
}

#[derive(Debug, Clone, Default)]
pub struct PacketOption {
    compression: bool,
    protocol: StoneTransferProtocol,
}