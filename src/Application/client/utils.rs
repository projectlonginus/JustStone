use crate::Application::malware::utils::shell::ShellStream;
use crate::stprotocol::utils::NormalSession;

pub struct Obsidian {
    pub session: NormalSession,
    pub exploits: ShellStream,
}