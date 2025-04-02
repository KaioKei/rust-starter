# Rust

## Documentation

Use these 2 docs in the first place, in the same time :

- [Rust official course](https://doc.rust-lang.org/reference/introduction.html)
- [Rust by examples (official)](https://doc.rust-lang.org/rust-by-example/index.html)

Learn about the basis of Rust and how it manages the memory :

- [Rust Ownership Concepts](./ownership.md)

Learn advanced practices :

- [Learn Rust with practical, production-tested guides](https://www.howtocodeit.com/articles)

Search engine for error codes, crates, docs, etc :

- [Query RS](https://query.rs/)

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

## Dependencies

Dependencies in Rust are called **crates**.
A crate is the smallest amount of code that the Rust compiler considers at a time.

A crate can come in one of two forms: 
- Binary crates are programs you can compile to an executable that you can run. Basically, they contain a `main` function.
- Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

For dependencies, check the official dependencies registry of Rust: `https://crates.io/`.  
You can also visit `https://docs.rs`

## build & Run

Cargo is Rust’s build system and package manager.  
Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

Read my [Cargo Guide](cargo.md).

Read the [official documentation](https://doc.rust-lang.org/cargo/)

## Linux container

Example using `./Containerfile`

```sh
podman build -f ./Containerfile -t localhost/rest-api-axum
```

Run with :

```sh
podman run -d --name axum -p 8080:8080 localhost/rest-api-axum 
curl -sSL http://localhost:8080
#> Hello, World !
```

## Troubleshooting

### Rustc VS Rustup vs Cargo

Rustc is the compiler.
Rustup is the toolchain manager (rust environments).
Cargo is the package manager.
