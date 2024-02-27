mod networks;
mod protocol;
mod utils;

pub use networks::*;
pub use protocol::*;
pub use utils::*;
use crate::structure::{StoneTransferProtocol, StructStone, Detector};

impl Handlers for Client {
    fn default_client_handler(&mut self) -> Result<(), ()> {
        loop {
            // 새션 생성후 서버와 지속적인 통신을 위한 루프문
            match self.receiving().get_type() {
                StoneTransferProtocol::Connection => {
                    println!("Connection OK");
                    Ok(())
                }
                // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
                StoneTransferProtocol::ExecuteCmd =>
                // 타입이 ExecuteCmd 일 경우
                    self.exploit(),

                StoneTransferProtocol::Download =>
                // 타입이 Download 일 경우
                    self.download(),

                StoneTransferProtocol::Upload =>
                // 타입이 Upload 일 경우
                    self.upload(),

                StoneTransferProtocol::Disconnect => {
                    self.disconnect();
                    break Ok(());
                } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

                _ => self.send(StructStone::default()), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
            }?
        }
    }
}