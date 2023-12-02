use std::io::{Read, Write};
use std::net::TcpStream;
use std::u8;
use crate::structure::{ StructStoneHeader, StructRawStonePayload, StructStone, StructStonePayload, Generator, Detector };

#[derive(Debug)]
pub struct Session {
    ip_port: String,
    socket: TcpStream,
}

impl Session {
    pub fn new(ip_port: String) -> Session {
        let mut socket;

        if let Ok(s) = TcpStream::connect(ip_port.clone()) {
            socket = s;

            let packet = StructRawStonePayload {
                sysinfo: String::from("sysinfo"),
                command_input: String::from(""),
                command_output: String::from(""),
                stone_chain: String::from(""),
            }.to_stone();

            socket.write_all(&packet.stone).expect("TODO: panic message");

            Session { ip_port, socket }
        } else {
            Self::new(ip_port)
        }
    }
}

pub trait Client {
    fn send(&mut self, stone: Vec<u8>);
    fn disconnect(&mut self);
    fn get_payload_size(&mut self, header: StructStoneHeader) -> usize;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> StructStone;

}

impl Client for Session {
    fn send(&mut self, stone: Vec<u8>) {
        self.socket.write_all(stone.as_slice()).expect("Failed to send");
    }

    fn disconnect(&mut self) {
        let packet = StructStone::disconnect();
        self.send(packet.stone);
        self.socket.try_clone().expect("Failed to close");
    }

    fn get_payload_size(&mut self, header: StructStoneHeader) -> usize {
        let length_bytes: &[u8] = &header.stone_size;
        let length = u32::from_le_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]);
        return length as usize
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {

        let mut buffer : Vec<u8> = vec![0; buffer_size];

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => buffer_size,
            Err(_) => buffer_size
        };

        buffer
    }

    fn receiving(&mut self, mut buffer: StructStone) -> StructStone { // 함수가 재귀적으로 호출돠기 때문에 빈 헤더, 페이로드를 입력받음, 기본 헤더의 페이로드 크기는 12바이트 고정임
        let mut header = StructStoneHeader::default(); // 응답을 받을 빈 헤더 구조체 생성
        let mut payload = StructStonePayload::default(); // 응답을 받을 빈 페이로드 구조체 생성

        if buffer.header.stone_size != vec![12,0,0,0] { // 만약 수신받은 데이터의 크기가 12 바이트가 아니라면
            let buffer_size: usize = self.get_payload_size( buffer.header.clone() ); // 헤더에서 페이로드 크기를 추출후

            payload = StructStonePayload::from(self.recv( buffer_size )); // 페이로드 크기만큼 데이터를 받고 구조체로 변환하여 빈 페이로드 구조체에 저장
            return StructStone::from(buffer.header, payload); // 헤더와 페이로드를 결합하여 구조체로 반환
        }

        header = StructStoneHeader::load(self.recv(12)); //만함수 인자로 입력받은 헤더의 페이로드 크기가 12바이트 (기본 헤더 ) 라면 새로운 헤더 (12바이트 고정)을 수신받고
        return self.receiving(StructStone::from(header, payload)); // 새로운 헤더를 재귀함수로 입력함 이 경우 재귀함수에서 if buffer.header.stone_size != vec![12,0,0,0] 문에 걸려서 페이로드를 수신받게 됨

    }
}




