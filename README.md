Rubik's Cube Solver in Rust
---------------------------

A multithreaded Rubik's cube solver implemented in Rust, exploring different move sequences concurrently to find solutions efficiently.
Features

- Concurrent solving using Rust's Rayon crate
- Utilizes multiple CPU cores for faster solving
- Basic implementation of a Rubik's cube solver algorithm
- Modular design separating cube representation and solving logic

Usage

To run the solver:

- Clone the repository
- Build the project: cargo build
- Run the solver: cargo run

Dependencies

- Rayon 1.10.0: For asynchronous multithreading support
- Rand 0.8.5: For scrambling the cube in an unpredictable manner

To build the project:

- "cargo build --release"

This will compile the project in release mode, which is recommended for performance-critical applications like cube solving.
