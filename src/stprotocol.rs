use std::io::{Read, Write};
use std::net::TcpStream;
use std::ptr::eq;
use std::u8;
use crate::structure::{StoneTransferProtocol, StructStoneHeader, StructRawStonePayload, StructStone, StructStonePayload, Generator, Detector};

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
                sysinfo: String::from("sysinfo.."),
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
        let packet = StructRawStonePayload {
            sysinfo: String::from(""),
            command_input: String::from(""),
            command_output: String::from(""),
            stone_chain: String::from(""),
        }.to_stone();
        println!("닫음 : {:?}", packet.header_type());
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

    fn receiving(&mut self, mut buffer: StructStone) -> StructStone {
        let mut header = StructStoneHeader::default();
        let mut payload = StructStonePayload::default();

        if buffer.header.stone_size != vec![12,0,0,0] {
            let buffer_size: usize = self.get_payload_size( buffer.header.clone() );

            payload = StructStonePayload::from(self.recv( buffer_size ));
            return StructStone::from(buffer.header, payload);
        }

        header = StructStoneHeader::load(self.recv(12));
        return self.receiving(StructStone::from(header, payload));

    }
}




