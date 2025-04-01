# Rustup

Use rustup to set the toolchain you want to use for development.

You can check the official rustup documentation : https://rust-lang.github.io/rustup/basics.html

## Concepts

Rustup installs **toolchains**.  
A “toolchain” is a complete installation of the Rust compiler (rustc) and related tools (like cargo).

Rustup has 3 **channels** :
- stable: sets the last stable toolchain
- beta: sets the last pre-stable toolchain
- nightly: sets the last toolchain in development 




## Install a toolchain

Update Rustup :

```sh
rustup self update
rustup update
```

To install a toolchain and change a rustc version :

```sh
# most recent stable version
rustup toolchain install stable
# most recent version
rustup toolchain install nightly
# specific previous version
rustup toolchain install 1.76.0
# list the installed toolchains
rustup toolchain list
```

Show current toolchain :

```sh
# The following command shows :
# - the current toolchain
# - the current rust compiler version (rustc)
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/kaio/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu
1.76.0-x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.78.0 (9b00956e5 2024-04-29)
```

Show current Rust compiler version installed by Rustup :

```sh
rustc --version
#> rustc 1.78.0
```

## Change the toolchain

Set the toolchain :

```sh
# Set the default toolchain of the system to 'stable'
rustup default stable
```

You can check what version the current channels are pointing to with :

```sh
rustup check
```

Use `override` to set a local toolchain in a directory :

```sh
# set
cd /tmp/test
rustup override set 1.76.0
rustup show active-toolchain
# list
rustup override list
/tmp/test    1.76.0-x86_64-unknown-linux-gnu
# check it under the /tmp/test folder
rustc --version
#> rustc 1.76.0
```

You can remove a local toolchain with : 

```sh
cd /tmp/test
rustup override unset
```

## Projects toolchains

Some projects find themselves pinned to a specific release of Rust and want this information reflected in their source
repository.

In these cases the toolchain can be named in the project’s directory in a file called rust-toolchain.toml or
rust-toolchain. If both files are present in a directory, the latter is used for backwards compatibility. The files use
the TOML format and have the following layout:

```toml
[toolchain]
# which toolchain to use (check with `rustup toolchain list`)
channel = "stable-x86_64-unknown-linux-gnu"
# The profile setting names a group of components to be installed. The value is a string.
# The valid options are: minimal, default, and complete.
# check the documentation to learn more about the profiles
# Note that if not specified, the default profile is not necessarily used, as a different default profile might
# have been set with rustup set profile.
profile = "default"
```

This is the minimal configuration for a rustup-toolchain file.
Check the official documentation for more configurations.
