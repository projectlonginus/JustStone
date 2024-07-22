use std::{io::{
    Error,
    Read,
    Write
}, io, mem::replace, net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs}};
use crate::{
    stprotocol::{
        utils::{
            HandleSessions,
            NormalSession,
            PacketProcessing
        }
    },
    structure::{
        utils::{
            enums::{HandshakeType, Packet},
            structs::define::{
                EncryptionInfo,
                StructStone,
                StructStoneHeader,
                StructStonePayload
            },
            traits::{Detector, PacketPreset},
        }
    },
};
use crate::stprotocol::utils::{NormalSessionLayer, SecureSession, SecureSessionLayer};

static PORT:u16 = 6974;

impl NormalSessionLayer for NormalSession {}

impl SecureSessionLayer for NormalSession {}

impl HandleSessions for NormalSession {
    fn new<A: ToSocketAddrs>(address: A, packet: Packet) -> Result<(TcpStream,Packet), (Error, Packet)> {
        packet.display();
        match TcpStream::connect(address) {
            Ok(mut socket ) => {
                return match socket.write_all(packet.take_stone().unwrap()) {
                    Ok(_) => Ok((socket, packet)),
                    Err(error)  => Err((error, packet)),
                }
            },
            Err(error) => Err((error, packet))
        }
    }

    fn normal(address: IpAddr, packet: Packet) -> NormalSession {
        Self::new(SocketAddr::new(address, PORT), packet)
            .map(|(socket,packet)| {
                println!("normal connection success.\n");
                NormalSession::set(socket).with_send_packet(packet)
            })
            .unwrap_or_else(|(Error, Packet)| {
                println!("A normal connection failed: {:?}.\nretry normal connection.\n", Error);
                Self::normal(address, Packet)
            })
    }

    // fn secure(&mut self) -> SecureSession {
    //     self.receiving(StructStone::buffer());
    //     // self.cipher.rsa.set_public_key(RsaCrypto::from_pub_key(self.recv_packet.take_file().unwrap()));
    //     todo!("CA 인증서 서명 인증 로직 추가")
    //     // Self::new(SocketAddr::new(address, PORT), packet)
    //     //     .map(|(socket,packet)| {
    //     //         println!("secure connection success.\n");
    //     //         NormalSession::set(EncryptionInfo::default_encryption(), socket, packet)
    //     //     })
    //     //     .unwrap_or_else(|(Error, Packet)| {
    //     //         println!("A secure connection failed: {:?}.\nretry secure connection.\n", Error);
    //     //         Self::secure(address, Packet)
    //     //     }) // 리펙토링 필요 *
    // }

    fn establish_connection(address: IpAddr, conn_type: EncryptionInfo, packet: Packet, attempts: u32) -> NormalSession { // 리팩터링 필요
        if attempts >= 3 {
            match conn_type.Handshake_Type {
                HandshakeType::NoHandshake => panic!("Failed to establish any connection after multiple attempts.\n"),
                _ => return Self::establish_connection(address, conn_type, packet, attempts),
            }
        }
        match Self::new(SocketAddr::new(address, PORT), packet) {
            Ok((socket, packet)) => {
                println!("{} connection success.\n", conn_type.Activated);
                NormalSession::set(socket).with_send_packet(packet)
            }
            Err((error, packet)) => {
                println!("A {:?} connection failed (attempt {}): {:?}.\nRetrying connection.\n",
                         conn_type.Type, attempts + 1, error);
                Self::establish_connection(address, conn_type, packet, attempts + 1)
            }
        }
    }

    fn optional(address: IpAddr, encryption: EncryptionInfo) -> NormalSession { // 리팩터링 필요
        let packet = match encryption.Activated {
            false => NormalSession::connection(),
            _ => SecureSession::connection()
        };
        Self::establish_connection(address, encryption, packet, 0)
    }

    fn is_connected(&self) -> bool {
        match self.socket.peek(&mut [0; 128]) {
            Ok(_) => true,
            Err(e) if e.kind() == io::ErrorKind::ConnectionReset => false,
            Err(_) => true,
        }
    }

    fn reconnection(&mut self) -> Result<NormalSession, Error> {
        self.send_disconnect()?;
        Ok(NormalSession::normal(self.socket.peer_addr().unwrap().ip(), self.connection()))
    }

    // fn encryption(&mut self) -> Result<(), ParseError> { 리펙터링 필요
    //     if !self.send_packet.is_encryption() {
    //         return Err(ParseError::Unimplemented("".to_string()));
    //     }
    //
    //     let packet = match &mut self.send_packet {
    //         Packet::StructStone(ref mut packet) => replace(packet, Default::default()),
    //         _ => return Err(ParseError::Unimplemented("Packet does not exist".to_string())),
    //     };
    //
    //     let temp_enc = EncryptionInfo::default_encryption();
    //
    //     let encrypted_packet = match packet.get_type() {
    //         Connection => {
    //             println!("핸드셰이크");
    //             SecureHandshakePacket::build(packet, &temp_enc)
    //                 .map(Packet::from)
    //                 .map_err(|error| error)?
    //         }
    //         _ => {
    //             println!("암호화");
    //             SecurePacket::build(packet, &temp_enc) // 리펙타링 필요
    //                 .map(Packet::from)
    //                 .map_err(|error| error)?
    //         }
    //     };
    //
    //     self.set_packet(encrypted_packet);
    //     Ok(())
    // }
    //
    // fn decryption(&mut self) -> Result<(), ParseError> {
    //     Ok(())
    // }

    fn send(&mut self) -> Result<Packet, Error> {
        self.take_socket()
            .write_all(self.send_packet.take_stone().unwrap())
            .map(|_| self.get_packet())
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {
        let mut buffer = vec![0; buffer_size];
        self.take_socket().read_exact(&mut buffer).expect("Failed to read from socket");
        buffer
    }

    fn receiving(&mut self, mut buffer: StructStone) -> &Packet {
        let mut payload = StructStonePayload::default();
        let buffer_size = buffer.get_size();

        if buffer_size != 12 {
            payload = StructStonePayload::load(self.recv(buffer_size));
            self.save_packet(Packet::from(StructStone::build(buffer.get_header(), payload)));
            return self.peek_packet();
        }

        let header = StructStoneHeader::load(self.recv(12));
        self.receiving(StructStone::build(header, payload))
    }

    fn send_disconnect(&mut self) -> io::Result<()> {
        self.set_packet(self.disconnect().unwrap().unwrap());
        self.send().map(|_| ())
    }
}
