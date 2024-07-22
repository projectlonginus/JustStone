#[cfg(test)]
mod tests {
    use crate::{structure::{
    utils::{
        enums::{EncryptionFlag, StatusCode, StoneTransferProtocol},
        traits::ProtocolCodec,
        structs::define::{SecureHandshakePacket, SecurePacket, StructStone}
    },
    packet::PacketPreset},
    stprotocol::utils::{NormalSession, SecureSession}
    };

    // #[test]
    // fn test_handshake() {
    //     let packet:StructStone = handshake().unwrap().unwrap();
    //     assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Handshake);
    //     assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Secured);
    //     assert!(packet.payload.is_empty());
    // }

    #[test]
    fn test_connection(session: NormalSession) {
        let packet:StructStone  = session.connection().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Connection);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert!(packet.payload.is_empty());
    }

    #[test]
    fn test_disconnect(session: NormalSession) {
        let packet:StructStone  = session.disconnect().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Disconnect);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert!(packet.payload.is_empty());
    }

    #[test]
    fn test_response(session: NormalSession) {
        let msg = "Test response";
        let packet:StructStone  = session.response(msg).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Response);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.response, msg.as_bytes());
    }

    #[test]
    fn test_download(session: NormalSession) {
        let file_name = vec![1, 2, 3, 4, 5];
        let packet:StructStone  = session.download(file_name.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Download);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.file, file_name);
    }

    #[test]
    fn test_upload(session: NormalSession) {
        let file_content = vec![5, 4, 3, 2, 1];
        let packet:StructStone  = session.upload(file_content.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Upload);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.file, file_content);
    }

    #[test]
    fn test_exploit(session: NormalSession) {
        let output = vec![0, 1, 0, 1];
        let packet:StructStone  = session.exploit(output.clone()).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::ExecuteCmd);
        assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Normal);
        assert_eq!(packet.payload.response, output);
    }

    #[test]
    fn test_secure_connection(ssl: SecureSession) {
        let packet:SecureHandshakePacket = ssl.connection().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Handshake);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        assert_eq!(EncryptionFlag::get_type(&packet.encryption_flag), EncryptionFlag::RAGS);
        assert!(packet.origin_packet.payload.is_empty());
    }

    #[test]
    fn test_secure_disconnect(ssl: SecureSession) {
        let packet:SecurePacket = ssl.disconnect().unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Disconnect);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        assert!(packet.origin_packet.payload.is_empty());
    }

    #[test]
    fn test_secure_response(ssl: SecureSession) {
        let msg = "Secure test response";
        let packet:SecurePacket = ssl.response(msg).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Response);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
        println!("{:?}", packet.encrypted_packet)
    }

    #[test]
    fn test_secure_download(ssl: SecureSession) {
        let file_content = vec![1, 2, 3, 4, 5];
        let packet:SecurePacket = ssl.download(file_content).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Download);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }

    #[test]
    fn test_secure_upload(ssl: SecureSession) {
        let file_content = vec![5, 4, 3, 2, 1];
        let packet:SecurePacket  = ssl.upload(file_content).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::Upload);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }

    #[test]
    fn test_secure_exploit(ssl: SecureSession) {
        let output = vec![0, 1, 0, 1];
        let packet:SecurePacket  = ssl.exploit(output).unwrap().unwrap();
        assert_eq!(StoneTransferProtocol::get_type(&packet.origin_packet.header.stone_type), StoneTransferProtocol::ExecuteCmd);
        assert_eq!(StatusCode::get_type(&packet.origin_packet.header.stone_status), StatusCode::Secured);
    }

    #[test]
    fn test_normal_vs_secure(ssl: SecureSession, session: NormalSession) {
        assert_ne!(session.connection()          , ssl.connection());
        assert_ne!(session.disconnect()          , ssl.disconnect());
        assert_ne!(session.response("test")      , ssl.response("test"));
        assert_ne!(session.download(vec![1,2,3]) , ssl.download(vec![1,2,3]));
        assert_ne!(session.upload(vec![1,2,3])   , ssl.upload(vec![1,2,3]));
        assert_ne!(session.exploit(vec![1,2,3])  , ssl.exploit(vec![1,2,3]));
    }
}