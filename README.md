# Learn Rust

Welcome to Faisal's journey of learning Rust programming language!

## Prerequisites

To install Rust, follow the documentation at [https://doc.rust-lang.org/book/ch01-01-installation.html](https://doc.rust-lang.org/book/ch01-01-installation.html). You can install Rust via `rustup` or UNIX-like operating system's package managers such as `dnf`, `pacman`, or `homebrew`. For installation on Windows, I personally prefer to install it via WSL (Windows Subsystem for Linux).

## How to compile

If the `build` directory doesn't exist yet, create it.

```bash
mkdir build
```

Make sure to compile the `rust` source code to the related `build` directory.

```bash
rustc path/to/file.rs -o path/to/build/file.o
```

## How to execute

To execute the binary executable, run this following command.

```bash
./path/to/build/file.o
```

