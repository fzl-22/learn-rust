# Learn Rust

Welcome to Faisal's journey of learning Rust programming language!

## Prerequisites

To install Rust, follow the documentation at [https://doc.rust-lang.org/book/ch01-01-installation.html](https://doc.rust-lang.org/book/ch01-01-installation.html). You can install Rust via `rustup` or any UNIX-like operating system's package managers such as `dnf`, `pacman`, or `homebrew`. For installation on Windows, I personally prefer to install it on WSL (Windows Subsystem for Linux).

## How to create project

To create `rust` project, use `cargo new {project-name}`.

```bash
cargo new my-project
```

## How to build

To build `rust` project, `cd` into the root project directory. Then, run `cargo build`. 


```bash
cargo build
```

This command will compile the entire project and store it in `target` directory.

## How to run

To run the target binary, just execute the binary in `target/debug/`.

```bash
./target/debug/{project-name}
```

## How to build and run

For simplicity, use `cargo run` in the root of project directory.

```bash
cargo run
```

