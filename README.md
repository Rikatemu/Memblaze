# Memblaze

Memblaze is a simple, in-memory database designed as an open-source learning project. Built using Rust, it focuses on providing a hands-on experience with asynchronous programming and basic database operations. With an easy-to-understand codebase that leverages `tokio` for async operations and `DashMap` for thread-safe data storage, Memblaze is perfect for those eager to explore Rust programming or understand the inner workings of in-memory databases.

## Features

- **Simple Design**: Ideal for beginners and those looking to deepen their understanding of Rust and asynchronous programming.
- **Asynchronous Operations**: Employs Rust's `tokio` runtime for efficient, non-blocking I/O operations.
- **Thread-Safe Data Storage**: Uses `DashMap` for safe, concurrent data access and manipulation.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo, Rust's package manager

### Installation

To get started with Memblaze, clone the repository and build the project:

```bash
git clone https://github.com/Rikatemu/Memblaze.git
cd memblaze
cargo build --release
```
### Running Memblaze

Launch the server with the following command:

```bash
cargo run --release
```

Memblaze listens on 127.0.0.1:6379. Use a TCP client like telnet to interact with the database:

```bash
telnet 127.0.0.1 6379
```

Supported Commands
- `SET key value`: Set the value of a key.
- `GET key`: Get the value of a key.
- `DEL key`: Delete a key.

Example usage:

```bash
SET mykey hello
GET mykey
hello
DEL mykey
```

### Contributing

We welcome contributions to Memblaze! Whether it's feature suggestions, bug reports, or pull requests, your input is valuable.

## License

Memblaze is released under the MIT License. See [LICENSE](LICENSE) for more information.
