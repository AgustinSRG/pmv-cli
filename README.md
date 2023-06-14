# Command line interface client for PersonalMediaVault

This project is a command line interface client to interact with [PersonalMediaVault](https://github.com/AgustinSRG/PersonalMediaVault).

For regular use cases, you may use the web interface instead. However, a CLi tool may be useful when:

 - Creating shell scripts that interact with the media vaults
 - Interacting with the vault when a GUI environment is not available.
 - For advanced tasks, like cloning media assets from one vault wio another.

This CLI tool is coded using the Rust programming language.

## Download

TODO

## Usage

In order to display the available options, type:

```sh
pmv-cli --help
```

Check the [manual](./MANUAL.md) for a detailed explanation of each available option.

## Build from source code

In order to build the source code, you will need the rust compiler.

Type the following command to compile:

```sh
cargo build
```

The resulting binaries will be placed in the `target` folder.
