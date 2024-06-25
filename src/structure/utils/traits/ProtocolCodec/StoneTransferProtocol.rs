use crate::structure::utils::enums::StoneTransferProtocol;
use crate::structure::utils::traits::define::ProtocolCodec;

impl ProtocolCodec for StoneTransferProtocol {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0, 0, 0, 0] => StoneTransferProtocol::Connection,
            [0, 0, 0, 1] => StoneTransferProtocol::Handshake,
            [0, 0, 1, 0] => StoneTransferProtocol::HealthCheck,
            [0, 0, 1, 1] => StoneTransferProtocol::Disconnect,

            [0, 1, 0, 0] => StoneTransferProtocol::ExecuteCmd,
            [0, 1, 0, 1] => StoneTransferProtocol::Upload,
            [0, 1, 1, 0] => StoneTransferProtocol::Download,
            [0, 1, 1, 1] => StoneTransferProtocol::Response,

            _ => StoneTransferProtocol::Unknown,
        }
    }

    fn to_bytes(&self) -> [u8; 4]
    {
        match self {
            StoneTransferProtocol::Connection   => [0, 0, 0, 0],
            StoneTransferProtocol::Handshake    => [0, 0, 0, 1],
            StoneTransferProtocol::HealthCheck  => [0, 0, 1, 0],
            StoneTransferProtocol::Disconnect   => [0, 0, 1, 1],

            StoneTransferProtocol::ExecuteCmd   => [0, 1, 0, 0],
            StoneTransferProtocol::Upload       => [0, 1, 0, 1],
            StoneTransferProtocol::Download     => [0, 1, 1, 0],
            StoneTransferProtocol::Response     => [0, 1, 1, 1],

            _ => [1,0,0,0],
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