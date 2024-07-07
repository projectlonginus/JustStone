mod Application;
mod stprotocol;
mod structure;
mod utility;

use crate::stprotocol::utils::Handlers;

fn main() {
    // let mut client = Application::Obsidian::secure("127.0.0.1");  // 핸드세이크, 암호화 통신 구조가 아직 확립되지 않음

    let mut client = Application::Obsidian::normal("127.0.0.1");
    client.default_client_handler()

    // Application::server::igneous::run_server()
}