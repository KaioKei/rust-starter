# Rustup

## Install a toolchain

To install a toolchain and change a rustc version :

```sh
# most recent stable version
rustup toolchain install stable
# most recent version
rustup toolchain install nightly
# specific previous version
rustup toolchain install 1.76.0
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

## Change the toolchain

Set the toolchain :

```sh
# Set the default toolchain of the system to 'stable'
rustup default stable
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
# Remove a local toolchain for a directory
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
