# Command line interface client for PersonalMediaVault

[![Rust](https://github.com/AgustinSRG/pmv-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/AgustinSRG/pmv-cli/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](./LICENSE)

This project is a command line interface client to interact with [PersonalMediaVault](https://github.com/AgustinSRG/PersonalMediaVault).

For regular use cases, you may use the web interface instead. However, a CLi tool may be useful when:

 - Creating shell scripts that interact with the media vaults
 - Interacting with the vault when a GUI environment is not available.
 - For advanced tasks, like cloning media assets from one vault to another.

This CLI tool is coded using the Rust programming language.

## Download

You can download the compiled binaries for this tool visiting the [Releases](https://github.com/AgustinSRG/pmv-cli/releases) section.

If you don't find any binaries for your system, you can try compiling it from source code.

## Usage

In order to display the available options, type:

```sh
pmv-cli --help
```

Check the [manual](./MANUAL.md) for a detailed explanation of each available option.

## Build from source code

In order to build the source code, you will need the rust compiler installed in your system.

Also, due to dependencies on libssl, you will need to install it:

```sh
sudo apt install libssl-dev
```

Type the following command to compile:

```sh
cargo build --release
```

The resulting binaries will be placed in the `target` folder.
