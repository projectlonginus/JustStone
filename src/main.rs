
mod exploit;
mod structure;
mod stprotocol;

use std::io::Error;
use exploit::*;
use structure::*;
use stprotocol::{ Session, Client };

fn main() {

    println!("클라이언트 시작됨");

    let mut client = Session::new("127.0.0.1:6974".to_string());
    let mut buffer = StructStone::default();
    buffer = client.receiving(buffer);

    println!("recv : {:?}", buffer.header.detect_header_type());

    match client.send(buffer.stone.as_slice()) {
        Ok(r) => println!("{} : {:?}",r , buffer.header.detect_header_type()),
        Err(e) => println!("{} : {:?}",e , buffer.header.detect_header_type())
    }

}
