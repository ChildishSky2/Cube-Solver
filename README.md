Rubik's Cube Solver in Rust
---------------------------

A multithreaded Rubik's cube solver implemented in Rust, exploring different move sequences concurrently to find solutions efficiently.
Features

- Concurrent solving using Rust's tokio crate
- Utilizes multiple CPU cores for faster solving
- Basic implementation of a Rubik's cube solver algorithm
- Modular design separating cube representation and solving logic

Usage

To run the solver:

    Clone the repository
    Build the project: cargo build
    Run the solver: cargo run

Dependencies

    tokio: For asynchronous multithreading support

License

This project is licensed under MIT. See LICENSE for details.
Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
Building

To build the project:

cargo build --release

This will compile the project in release mode, which is recommended for performance-critical applications like cube solving.
