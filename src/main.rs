mod Application;
mod stprotocol;
mod structure;
mod utility;

use crate::stprotocol::utils::Handlers;

fn main() {
    let mut client = Application::Obsidian::secure("127.0.0.1");

    client.default_client_handler()

    // Application::server::igneous::run_server()
}