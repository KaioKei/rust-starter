# Cargo

## Check

To check your code without building :

```sh
# check everything
cargo check
# check only the sources that concern the future compiled bins
cargo check --bins
# check only the sources that concern one future compiled bin
cargo check --bin hello-world
```

## Build

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

## Install

To build without development flags and dependencies, use :

```sh
cargo build --bin hello-world --release
```

The binaries will be located in `target/release`.

## Clean

To remove all the targets (built binaries) :

````sh
cargo clean
````

