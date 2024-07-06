use crate::Application::malware::utils::shell::ShellStream;
use crate::stprotocol::utils::Session;

pub struct Obsidian {
    pub session: Session,
    pub exploits: ShellStream,
}