use std::{
    io::{self, Read, Write},
    mem::replace,
    net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs},
};
use std::io::Error;
use utils::Session;

use crate::{
    stprotocol::{HandleSession, utils},
    structure::{
        packet::{connection, disconnect, secure_connection, secure_disconnect},
        utils::{
            enums::{EncryptType, HandshakeType, Packet, ParseError, StoneTransferProtocol::Connection},
            structs::define::{SecureHandshakePacket, SecurePacket, StructStone, StructStonePayload},
            traits::define::Detector,
        },
    }
};
use crate::structure::utils::structs::define::{EncryptionInfo, StructStoneHeader};

impl HandleSession for Session {
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> io::Result<TcpStream> {
        TcpStream::connect(address).and_then(|mut socket| {
            socket.write_all(packet.take_stone().unwrap())?;
            Ok(socket)
        })
    }

    fn normal(address: IpAddr) -> Session {
        Self::new(SocketAddr::new(address, 6974), connection())
            .map(|socket| Session::set(EncryptionInfo::default(), socket, connection()))
            .unwrap_or_else(|error| panic!("normal connection error: {:?}", error))
    }

    fn secure(address: IpAddr, encryption: EncryptionInfo) -> Session {
        Self::new(SocketAddr::new(address, 6974), secure_connection(&encryption.Handshake_Type))
            .map(|socket| Session::set(encryption, socket, secure_connection(&HandshakeType::RSA)))
            .unwrap_or_else(|error| panic!("secure connection error: {:?}", error))
    }

    fn is_connected(&self) -> bool {
        match self.socket.peek(&mut [0; 128]) {
            Ok(_) => true,
            Err(e) if e.kind() == io::ErrorKind::ConnectionReset => false,
            Err(_) => true,
        }
    }

    fn reconnection(&mut self) -> Result<Session, Error> {
        if !self.is_connected() {
            let ip = self.socket.peer_addr().unwrap().ip();
            return match self.encryption.Type {
                EncryptType::NoEncryption => Ok(Session::normal(ip)),
                _ => Ok(Session::secure(
                    ip,
                    replace(&mut self.encryption, Default::default())
                )),
            };
        }

        self.send_disconnect()?;
        self.reconnection()
    }

    fn encryption(&mut self) -> Result<(), ParseError> {
        if !self.packet.is_encryption() {
            return Err(ParseError::Unimplemented("".to_string()));
        }

        let packet = match &mut self.packet {
            Packet::StructStone(ref mut packet) => replace(packet, Default::default()),
            _ => return Err(ParseError::Unimplemented("Packet does not exist".to_string())),
        };

        println!("{:?}", packet.get_type());

        let encrypted_packet = match packet.get_type() {
            Connection => {
                println!("핸드셰이크");
                SecureHandshakePacket::build(packet, &self.encryption)
                    .map(Packet::from)
                    .map_err(|error| error)?
            }
            _ => {
                println!("암호화");
                SecurePacket::build(packet, &self.encryption)
                    .map(Packet::from)
                    .map_err(|error| error)?
            }
        };

        self.set_packet(encrypted_packet);
        Ok(())
    }

    fn decryption(&mut self) -> Result<(), ParseError> {
        Ok(())
    }

    fn send(&mut self) -> Result<Packet, Error> {
        if self.packet.is_encryption() {
            println!("암호화 할거임");
            self.encryption().expect("Packet encryption failed.");
        }

        self.take_socket()
            .write_all(self.packet.take_stone().unwrap())
            .map(|_| self.get_packet())
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {
        let mut buffer = vec![0; buffer_size];
        self.take_socket().read_exact(&mut buffer).expect("Failed to read from socket");
        buffer
    }

    fn receiving(&mut self, mut buffer: StructStone) -> Packet {
        let mut payload = StructStonePayload::default();
        let buffer_size = buffer.get_size();

        if buffer_size != 12 {
            payload = StructStonePayload::load(self.recv(buffer_size));
            self.set_packet(Packet::from(StructStone::build(buffer.get_header(), payload)));
            return self.get_packet();
        }

        let header = StructStoneHeader::load(self.recv(12));
        self.receiving(StructStone::build(header, payload))
    }

    fn send_disconnect(&mut self) -> io::Result<()> {
        let packet = match self.encryption.Type {
            EncryptType::NoEncryption => disconnect(),
            _ => secure_disconnect(),
        };
        self.set_packet(packet);
        self.send().map(|_| ())
    }
}
