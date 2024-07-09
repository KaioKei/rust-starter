# Rust

## Documentation

Use these 2 docs in the first place, in the same time :

- [Rust official course](https://doc.rust-lang.org/reference/introduction.html)
- [Rust by examples (official)](https://doc.rust-lang.org/rust-by-example/index.html)

Learn about the basis of Rust and how it manages the memory :

- [Rust Ownership Concepts](./ownership.md)

## Install Rust

It is recommended to install Rust with `rustup` :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Environment

Rustup is the toolchain manager.
It works similarly as Pyenv, Goenv, etc .. (toolchain = environment).

Read my [Rustup Guide](rustup.md).

Read the [official documentation](https://rust-lang.github.io/rustup/)

## Build

Cargo is Rust’s build system and package manager.  
Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Read my [Cargo Guide](cargo.md).

Read the [official documentation](https://doc.rust-lang.org/cargo/)

### Run

Run binaries using the cargo environment (**recommended**):

```sh
cargo run --bin hello
```

Or run using the binaries directly :

```sh
# debug builds
./target/debug/hello
# release builds
./target/release/hello
```

## Troubleshooting

### Rustc VS Rustup vs Cargo

Rustc is the compiler.
Rustup is the toolchain manager (rust environments).
Cargo is the package manager.
