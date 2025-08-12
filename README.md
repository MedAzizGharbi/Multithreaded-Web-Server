# Multithreaded Web Server (Rust)

A simple multithreaded web server built in Rust by following [The Rust Programming Language Book](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html).  
The server uses a thread pool to handle multiple connections concurrently.

## Features
- Handles multiple HTTP requests simultaneously
- Graceful shutdown after a set number of requests
- Lightweight and minimal dependencies
- Demonstrates Rust concurrency patterns

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Cargo package manager (comes with Rust)

### Installation & Running
```bash
# Clone the repository
git clone https://github.com/<your-username>/<repo-name>.git
cd <repo-name>

# Build the project
cargo build --release

# Run the server
cargo run
```

By default, the server listens on 127.0.0.1:7878.
You can visit it in your browser:
http://127.0.0.1:7878

# How It Works
-The server binds to a TCP listener on a given address.
-A thread pool is created to manage a fixed number of worker threads.
-Each incoming request is sent to the pool for processing.
-The server serves basic HTML files and shuts down after handling a preset number of requests (configurable in code).
