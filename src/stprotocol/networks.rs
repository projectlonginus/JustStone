use std::{
    io::{Read, Write},
    net::TcpStream,
    u8
    ,
};

use utils::Session;

use crate::{
    stprotocol::{HandleSession, utils},
    structure::{
        connection,
        enums::{EncryptType::NotEncryption, Packet, StoneTransferProtocol::Connection},
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
            let packet = match Packet::unwrap(connection()) {
                Ok(p) => p,
                _ => StructStonePayload::build(false, NotEncryption, Connection, vec![]).raw_packet()
            };
            socket
                .write_all(packet.get_stone().unwrap())
                .expect("TODO: panic message");
            Session::set(NotEncryption, socket, Packet::from(packet))
        } else {
            Self::new(address)
        }
    }
}

impl HandleSession for Session {
    fn encryption(&mut self) -> std::io::Result<()> {
        match self.take_packet() {
            Packet::StructStone(payload) => {
                match &payload.get_type() {
                    Connection => todo!(),
                    _ => Ok(()),
                }
            }
            _ => Ok(())
        }
    }

    fn decryption(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn send(&mut self) -> std::io::Result<&Packet> {
        if self.is_encryption() {
            self.encryption().expect("Packet encryption failed.");
        }

        match self.take_socket().write_all(self.take_packet().get_stone().unwrap()) {
            _ => Ok(self.take_packet()),
        }
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];
        match self.take_socket().read_exact(&mut buffer) {
            Ok(_) => buffer_size,
            Err(e) => panic!(e)
        };

        buffer
    }

    fn receiving(&mut self, buffer: StructStone) -> Packet {
        // 함수가 재귀적으로 호출돠기 때문에 빈 헤더, 페이로드를 입력받음, 기본 헤더의 페이로드 크기는 12바이트 고정임
        let mut payload = StructStonePayload::default(); // 응답을 받을 빈 페이로드 구조체 생성
        let buffer_size = buffer.get_size();

        if buffer_size != 12 {
            // 만약 수신받은 데이터의 크기가 12 바이트가 아니라면
            payload = StructStonePayload::load(self.recv(buffer_size)); // 페이로드 크기만큼 데이터를 받고 구조체로 변환하여 빈 페이로드 구조체에 저장
            self.set_packet(Packet::from(StructStone::build(buffer.get_header(), payload)));
            return self.get_packet();
        }

        let header = crate::structure::structs::define::StructStoneHeader::load(self.recv(12)); //만함수 인자로 입력받은 헤더의 페이로드 크기가 12바이트 (기본 헤더 ) 라면 새로운 헤더 (12바이트 고정)을 수신받고
        return self.receiving(StructStone::build(header, payload)); // 새로운 헤더를 재귀함수로 입력함 이 경우 재귀함수에서 buffer_size != 12 문에 걸려서 페이로드를 수신받게 됨
    }
}
