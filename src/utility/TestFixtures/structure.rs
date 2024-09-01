use crate::{
    structure::{
        utils::{
            enums::{EncryptionFlag, Packet, StatusCode, StoneTransferProtocol},
            structs::define::{SecureHandshakePacket, SecurePacket, StructStone, StructStonePayload},
            traits::{PacketPreset, PacketTest, ProtocolCodec}
        }
    },
    utility::TestFixtures::stprotocol::{TestSession, TestSSL}
};

impl PacketPreset for TestSession {
    fn connection() -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Connection, vec![]).packet()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::NoEncryption, StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}
impl PacketPreset for TestSSL {

    fn connection() -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Handshake, vec![]).handshake_packet().unwrap()
    }

    fn disconnect(&self) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Disconnect, vec![]).packet()
    }

    fn response(&self, msg: &str) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Response, msg).packet()
    }

    fn download(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Download, file).packet()
    }

    fn upload(&self, file: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::Upload, file).packet()
    }

    fn exploit(&self, output: Vec<u8>) -> Packet {
        StructStonePayload::build(false, EncryptionFlag::RAGS, StoneTransferProtocol::ExecuteCmd, output).packet()
    }
}

impl PacketTest for TestSession {
    #[test]
    fn connectionTest() {
        let packet: StructStone = TestSession::connection().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Connection);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert!(packet.payload.is_empty());
    }

    #[test]
    fn disconnectTest(&self) {
        let packet: StructStone = self.disconnect().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Disconnect);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert!(packet.payload.is_empty());
    }

    #[test]
    fn responseTest(&self) {
        let msg = "Test response";
        let packet: StructStone = self.response(msg).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Response);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.response, msg.as_bytes());
    }

    #[test]
    fn downloadTest(&self) {
        let file_name = vec![1, 2, 3, 4, 5];
        let packet: StructStone = self.download(file_name.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Download);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.file, file_name);
    }

    #[test]
    fn uploadTest(&self) {
        let file_content = vec![5, 4, 3, 2, 1];
        let packet: StructStone = self.upload(file_content.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Upload);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.file, file_content);
    }

    #[test]
    fn exploitTest(&self) {
        let output = vec![0, 1, 0, 1];
        let packet: StructStone = self.exploit(output.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::ExecuteCmd);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.response, output);
    }
}

impl PacketTest for TestSSL {
    #[test]
    fn connectionTest(&self) {
        let packet:SecureHandshakePacket = self.connection().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Handshake);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        assert_eq!(EncryptionFlag::get_type(&packet.encryption_flag), EncryptionFlag::RAGS);
        assert!(packet.origin_packet.payload.is_empty());
    }

    #[test]
    fn disconnectTest(&self) {
        let packet:SecurePacket = self.disconnect().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Disconnect);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        assert!(packet.origin_packet.payload.is_empty());
    }

    #[test]
    fn responseTest(&self) {
        let msg = "Secure test response";
        let packet:SecurePacket = self.response(msg).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Response);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        println!("{:?}", packet.encrypted_packet)
    }

    #[test]
    fn downloadTest(&self) {
        let file_content = vec![1, 2, 3, 4, 5];
        let packet:SecurePacket = self.download(file_content).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Download);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }

    #[test]
    fn uploadTest(&self) {
        let file_content = vec![5, 4, 3, 2, 1];
        let packet:SecurePacket  = self.upload(file_content).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Upload);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }

    #[test]
    fn exploitTest(&self) {
        let output = vec![0, 1, 0, 1];
        let packet:SecurePacket  = self.exploit(output).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::ExecuteCmd);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }
}