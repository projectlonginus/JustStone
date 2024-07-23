use crate::stprotocol::utils::SecureSession;
use crate::structure::utils::enums::{EncryptionFlag, Packet, StoneTransferProtocol};
use crate::structure::utils::structs::define::StructStonePayload;
use crate::structure::utils::traits::PacketPreset;
use crate::utility::TestFixtures::stprotocol::TestSSL;

#[cfg(test)]
mod tests {
    use crate::{
        structure::{
            utils::{
                enums::{EncryptionFlag, StatusCode, StoneTransferProtocol},
                traits::{
                    ProtocolCodec,
                    PacketTest
                },
                structs::define::{SecureHandshakePacket, SecurePacket, StructStone},
            },
            packet::PacketPreset,
        },
        stprotocol::{
            utils::{
                NormalSession,
                SecureSession,
                NormalSessionLayer
            }
        },
        utility::TestFixtures::stprotocol::{TestSession, TestSSL}
    };
    use crate::structure::utils::enums::Packet;
    use crate::structure::utils::structs::define::StructStonePayload;

    // #[test]
    // fn test_handshake() {
    //     let packet:StructStone = handshake().unwrap().unwrap();
    //     assert_eq!(StoneTransferProtocol::get_type(&packet.header.stone_type), StoneTransferProtocol::Handshake);
    //     assert_eq!(StatusCode::get_type(&packet.header.stone_status), StatusCode::Secured);
    //     assert!(packet.payload.is_empty());
    // }


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