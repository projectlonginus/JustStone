use std::convert::Into;
use egui::ahash::HashSet;
use uuid::Uuid;
use crate::Application::server::utils::{Server, SessionInfo, SessionsSetup};
use crate::utility::secure::utils::{AesGcmSivCrypto, RsaCrypto};
use std::net::{Ipv4Addr, TcpStream};

impl Server {
    pub fn new() -> Server {
        Server {
            max_sessions: 0,
            max_threads: 0,
            default_session_setup: SessionsSetup::new(),
            session_list: Default::default(),
            sessions: Default::default(),
            ipv4addr: Default::default()
        }
    }

    pub fn set<T: Into<Server>>(&mut self, element: &str, value: T) {
        match element {
            "max_sessions" => self.max_sessions = value,
            "max_threads" => self.max_threads = value,
            "default_session_setup" => self.default_session_setup = value,
            "session_list" => self.session_list = value,
            "sessions" => self.sessions = value,
            "ipv4addr" => self.ipv4addr = value,
            _ => {}
        }
    }

    pub fn run() {
        todo!()
    }
}

impl SessionsSetup {
    pub fn new() -> SessionsSetup {
        SessionsSetup {
            use_encryption: false,
            use_compression: false,
            aes_cipher: AesGcmSivCrypto::new(),
            rsa_cipher: RsaCrypto::new(),
        }
    }
}

impl SessionInfo {
    pub fn new() -> SessionInfo {
        SessionInfo {
            session_id: Default::default(),
            ipv4addr: Default::default(),
            socket: Default::default(),
            setup: SessionsSetup::new(),
        }
    }
}