use std::net::TcpStream;
use crate::exploit::Exploits;
use crate::structure::{Detector, StructStone};

#[derive(Debug)]
pub struct Session {
    socket: TcpStream,
    packet: StructStone,
}

#[derive(Debug)]
pub struct Client {
    pub session: Session,
    pub exploits: Exploits,
}

impl Session {
    pub fn take_packet(&self) -> &StructStone {
        &self.packet
    }

    pub fn get_packet(&self) -> StructStone {
        self.packet.clone()
    }

    pub fn set_packet(&mut self, packet: StructStone) {
        self.packet = packet
    }

    pub fn take_socket(&self) -> &TcpStream {
        &self.socket
    }

    pub fn set(socket: TcpStream, packet: StructStone) -> Session {
        Session { socket, packet }
    }
}


impl Client {
    pub fn set_packet(&mut self, packet: StructStone) {
        self.session.set_packet(packet)
    }

    pub fn take_sysinfo(&self) -> &Vec<u8> { self.session.take_packet().take_sysinfo() }

    pub fn take_command(&self) -> &Vec<u8> {
        self.session.take_packet().take_command()
    }

    pub fn take_response(&self) -> &Vec<u8> {
        self.session.take_packet().take_response()
    }

    pub fn take_file(&self) -> &Vec<u8> {
        &self.session.take_packet().take_file()
    }

    pub fn get_sysinfo(&self) -> Vec<u8> { self.session.take_packet().get_sysinfo() }

    pub fn get_command(&self) -> Vec<u8> {
        self.session.take_packet().get_command()
    }

    pub fn get_response(&self) -> Vec<u8> {
        self.session.take_packet().get_response()
    }

    pub fn get_file(&self) -> Vec<u8> {
        self.session.take_packet().get_file()
    }
}

pub trait HandleSession {
    fn send(&mut self) -> Result<(), ()>;
    fn recv(&mut self, buffer_size: usize) -> Vec<u8>;
    fn receiving(&mut self, buffer: StructStone) -> &StructStone;
}

pub trait HandleProtocols {
    fn response(&mut self, msg: &str) -> Result<(), ()>;
    fn disconnect(&mut self);
    fn download(&mut self) -> Result<(), ()>;
    fn upload(&mut self) -> Result<(), ()>;
    fn exploit(&mut self) -> Result<(), ()>;
}

pub trait Handlers {
    fn default_client_handler(&mut self) -> Result<(), ()>;
}