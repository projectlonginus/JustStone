
mod exploit;
mod structure;
mod stprotocol;

use exploit::*;
use structure::*;
use stprotocol::{ Session, Client };

fn main() {

    let mut client = Session::new("127.0.0.1:6974".to_string());
    let mut buffer = StructStone::default();
    buffer = client.receiving(buffer);

    println!("받은거 : {:?}", buffer.header);
    println!("받은거 : {:?}", buffer.payload);

    // match client.recv() {
    //     Ok(ssh) => {
    //         // Access ssh.stone_status, ssh.stone_type, and ssh.stone_size here
    //         println!("받은거 : {:?} \n받은거 : {:?}", ssh.header, ssh.payload);
    //     },
    //     Err(_) => {
    //         // Handle the error case
    //         println!("Error receiving stone.");
    //     }
    // }
}
