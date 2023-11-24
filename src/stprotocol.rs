use std::io::{Read, Write};
use std::net::TcpStream;
use std::ptr::eq;
use std::u8;
use crate::structure::{StructStoneHeader, StructRawStonePayload, StructStone, StructStonePayload, Generator};

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
                sysinfo: String::from("sysinfo.."),
                command_input: String::from("command_input.."),
                command_output: String::from("command_output.."),
                stone_chain: String::from("stone_chain.."),
            }.generator();


            socket.write_all(&packet.stone).expect("TODO: panic message");


            Session { ip_port, socket }
        } else {
            Self::new(ip_port)
        }
    }

    pub fn set() {

    }
}

pub trait Client {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error>;
    fn detect_header_type(&mut self, header: Vec<u8>) -> bool;
    fn get_payload_size(&mut self, header: StructStoneHeader) -> usize;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> StructStone;

}

impl Client for Session {
    fn send_stone(&mut self, stone: &[u8]) -> Result<(), std::io::Error> {
        self.socket.write_all(stone)?;
        Ok(())
    }

    fn detect_header_type(&mut self, header: Vec<u8>) -> bool {
        todo!()
    }

    fn get_payload_size(&mut self, header: StructStoneHeader) -> usize {
        let length_bytes: &[u8] = &header.stone_size;
        let length = u32::from_le_bytes([length_bytes[0], length_bytes[1], length_bytes[2], length_bytes[3]]);
        return length as usize
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {

        let mut buffer : Vec<u8> = vec![0; buffer_size];

        println!("버퍼 생성 성공! 버퍼 크기 : {}", buffer.len());

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => println!("성공적으로 수신함! 페이로드 크기 : {}", buffer_size),
            Err(_) => println!("에러 발생! 페이로드 크기 : {}", buffer_size)
        }

        buffer
    }

    fn receiving(&mut self, mut buffer: StructStone) -> StructStone {
        let mut header = StructStoneHeader::default();
        let mut payload = StructStonePayload::default();
        let mut packet: Vec<u8> = Vec::new();

        println!("{:?}, {}, {}, {}", buffer.header, buffer.header.eq(&header), buffer.payload.eq(&payload), buffer.header.stone_size != vec![12]);

        if buffer.header.stone_size != vec![12,0,0,0] {
            let buffer_size: usize = self.get_payload_size( buffer.header );
            println!("페이로드 크기 추정 완료! : {}", buffer_size);

            payload = StructStonePayload::from(self.recv( buffer_size ));
            return StructStone::from(header, payload)
        }

        header = StructStoneHeader::load(self.recv(12));
        return self.receiving(StructStone::from(header, payload));

    }
}




