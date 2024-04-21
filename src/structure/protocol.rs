use crate::structure::utils::StoneTransferProtocol;

impl StoneTransferProtocol {
    pub fn type_check(vec: &Vec<u8>) -> StoneTransferProtocol {
        match vec.as_slice() {
            &[0, 0, 0, 0] => StoneTransferProtocol::Connection,
            &[1, 0, 0, 0] => StoneTransferProtocol::Handshake,
            &[4, 0, 0, 0] => StoneTransferProtocol::HealthCheck,
            &[5, 0, 0, 0] => StoneTransferProtocol::Disconnect,

            &[2, 0, 0, 0] => StoneTransferProtocol::ExecuteCmd,
            &[7, 0, 0, 0] => StoneTransferProtocol::Upload,
            &[8, 0, 0, 0] => StoneTransferProtocol::Download,
            &[3, 0, 0, 0] => StoneTransferProtocol::Response,

            _ => StoneTransferProtocol::Unknown,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        {
            match self {
                StoneTransferProtocol::Connection => vec![0, 0, 0, 0],
                StoneTransferProtocol::Handshake => vec![1, 0, 0, 0],
                StoneTransferProtocol::HealthCheck => vec![4, 0, 0, 0],
                StoneTransferProtocol::Disconnect => vec![5, 0, 0, 0],

                StoneTransferProtocol::ExecuteCmd => vec![2, 0, 0, 0],
                StoneTransferProtocol::Upload => vec![7, 0, 0, 0],
                StoneTransferProtocol::Download => vec![8, 0, 0, 0],
                StoneTransferProtocol::Response => vec![3, 0, 0, 0],

                StoneTransferProtocol::Unknown => vec![0, 0, 0, 1],
                _ => vec![0, 0, 0, 2],
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StoneTransferProtocol::Connection => "Connection".to_string(),
            StoneTransferProtocol::Handshake => "Handshake".to_string(),
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
