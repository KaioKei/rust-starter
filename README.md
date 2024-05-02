# Rust

## Install Rust

It is recommended to install Rust with `rustup` :

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Manage environment

Rustup is the toolchain manager.
It works similarly as Pyenv, Goenv, etc .. (toolchain = environment).

Read the [Rustup Guide](rustup.md).
Read the [official documentation](https://rust-lang.github.io/rustup/)

## Build with Cargo

Cargo is Rust’s build system and package manager.  
Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and
building those libraries.

To build a binary of your code, edit the `Cargo.toml` file :

```toml
# ...

[[bin]]
# name of your binary
name = "hello-world"
# path to the entry point
path = "src/main.rs"
```

Then execute :

```sh
# build all the binaries defined in Cargo.toml
cargo build --bins
# build a specific binary
cargo build --bin hello-world
# build for release
# exclude all development flags and deps
cargo build --bin hello-world --release
```

The binaries are located in `target/debug/` by default.  
With `--relase`, the binaries are located in `target/release`.

Targets correspond to deployment environments.  

## Troubleshooting

### Rustc VS Rustup vs Cargo

Rustc is the compiler.
Rustup is the toolchain manager (rust environments).
Cargo is the package manager.
