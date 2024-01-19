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

The backdoor is divided into four files, each responsible for specific functionalities:

1. **main.rs:** Contains the core functionalities of the backdoor.
2. **exploits.rs:** Manages the implementation of backdoor exploits.
3. **stprotocol.rs:** Houses the backdoor's proprietary protocol, supporting features like file upload, file download, and command execution.
4. **structure.rs:** Provides packet configuration for the protocol, including packet conversion, serialization, and deserialization.

### Customization

If you wish to customize the backdoor, follow these guidelines:

1. Open the relevant file for customization:
   - `exploits.rs` for exploit customization.
   - `stprotocol.rs` for protocol customization.
   - `structure.rs` for packet structure customization.

2. Make your desired modifications.

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
