use std::{
    io::{Read, Write},
    net::{
        Shutdown,
        TcpStream,
    },
    thread,
    time::Duration,
    u8,
};

use utils::Session;

use crate::{
    stprotocol::{HandleSession, utils},
    structure::{
        connection,
        enums::{Packet, StoneTransferProtocol},
        structs::define::{
            StructStone,
            StructStonePayload,
        },
        traits::define::Detector,
    },
};

impl Session {
    pub fn new(address: &str) -> Session {
        if let Ok(mut socket) = TcpStream::connect(address) {
            let packet = connection();
            socket
                .write_all(&packet.get_stone())
                .expect("TODO: panic message");
            Session::set(false, socket, packet)
        } else {
            Self::new(address)
        }
    }

    pub fn handle_connection_loss(&self) {
        self.take_socket().shutdown(Shutdown::Both).ok();
        loop {
            println!("handle_connection_loss");
            if let Ok(mut socket) =
                TcpStream::connect(self.take_socket().local_addr().unwrap().ip().to_string())
            {
                match socket.write_all(self.take_packet().get_stone()) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
            thread::sleep(Duration::from_micros(1000))
        }
    }
}

impl HandleSession for Session {
    fn encryption(&mut self) -> std::io::Result<()> {
        match self.take_packet() {
            Packet::StructStone { header, .. } => {
                match StoneTransferProtocol::type_check(&header.stone_type) {
                    StoneTransferProtocol::Connection => self.set_packet(self.secure_packet().expect("Packet::secure_packet()")),
                    _ => Ok(()),
                }
            }
            _ => Ok(())
        }
    }

    fn decryption(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn send(&mut self) -> Result<&StructStone, &StructStone> {
        if self.is_encryption() {
            self.encryption().expect("Packet encryption failed.");
        }

        match self.take_socket().write_all(self.take_packet().get_stone()) {
            Ok(_) => Ok(self.take_packet()),
            Err(_) => Err(self.take_packet()),
        }
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        match self.take_socket().read_exact(&mut buffer) {
            Ok(_) => buffer_size,
            Err(_) => buffer_size,
        };

        buffer
    }

    fn receiving(&mut self, buffer: StructStone) -> StructStone {
        // 함수가 재귀적으로 호출돠기 때문에 빈 헤더, 페이로드를 입력받음, 기본 헤더의 페이로드 크기는 12바이트 고정임
        let mut payload = StructStonePayload::default(); // 응답을 받을 빈 페이로드 구조체 생성
        let buffer_size = buffer.get_size();

        if buffer_size != 12 {
            // 만약 수신받은 데이터의 크기가 12 바이트가 아니라면
            payload = StructStonePayload::load(self.recv(buffer_size)); // 페이로드 크기만큼 데이터를 받고 구조체로 변환하여 빈 페이로드 구조체에 저장
            self.set_packet(Packet::from(StructStone::build(buffer.get_header(), payload)));
            return Packet::to_packet_type(self.take_packet()).expect("Packet::to_packet_type(self.take_packet())");
        }

        let header = crate::structure::structs::define::StructStoneHeader::load(self.recv(12)); //만함수 인자로 입력받은 헤더의 페이로드 크기가 12바이트 (기본 헤더 ) 라면 새로운 헤더 (12바이트 고정)을 수신받고
        return self.receiving(StructStone::build(header, payload)); // 새로운 헤더를 재귀함수로 입력함 이 경우 재귀함수에서 buffer_size != 12 문에 걸려서 페이로드를 수신받게 됨
    }
}
