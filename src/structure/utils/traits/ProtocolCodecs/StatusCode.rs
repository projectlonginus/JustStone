use crate::structure::utils::enums::StatusCode;
use crate::structure::utils::traits::define::ProtocolCodec;

impl ProtocolCodec for StatusCode {
    fn get_type(vec: &[u8; 4]) -> Self {
        match vec {
            [0, 0, 0, 0] => StatusCode::Normal,
            [0, 0, 0, 1] => StatusCode::Compressed,
            [0, 0, 1, 0] => StatusCode::Secured,
            [0, 0, 1, 1] => StatusCode::SCPacket,
            _ => StatusCode::Modulated
        }
    }

    fn to_bytes(&self) -> [u8; 4] {
        match self {
            StatusCode::Normal      => [0, 0, 0, 0],
            StatusCode::Secured     => [0, 0, 0, 1],
            StatusCode::Compressed  => [0, 0, 1, 0],
            StatusCode::SCPacket    => [0, 0, 1, 1],
            _ => [0,1,0,0]
        }
    }

    fn to_string(&self) -> String {
        match self {
            StatusCode::Normal => "Normal".to_string(),
            StatusCode::Secured => "Secured".to_string(),
            StatusCode::Compressed => "Compressed".to_string(),
            StatusCode::SCPacket => "SCPacket".to_string(),
            _ => "".to_string()
        }
    }
}