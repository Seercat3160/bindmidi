# midi2key

Bind MIDI notes to keyboard keys and mouse movement.

Midi2key is a simple program, written in Rust, to allow MIDI notes to trigger typing of keys, strings and movement of the mouse and scroll wheel.

[![Debug Build](https://github.com/Seercat3160/midi2key/actions/workflows/rust-debug.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/rust-debug.yml)
[![Release Build](https://github.com/Seercat3160/midi2key/actions/workflows/rust-release.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/rust-release.yml)

## Installation

### Pre-built binaries

You can visit [GitHub Releases](https://github.com/Seercat3160/midi2key/releases) to download the latest release binary. These should be ready to use (although make sure to check the description of the release for any known quirks).

If you want to use the latest version available, visit [Actions](https://github.com/Seercat3160/midi2key/actions) to download a binary for any commit. These will almost certainly be a work-in-progress with features half-baked, so steer clear unless you know it's what you need.

### Building from Source

You can also build from source, for example if you don't trust the pre-made releases or you want to use an unsupported platform. It's a standard project using Cargo, so install Rust with `rustup`, clone the repo or get a release's source archive, and run `cargo build --release`. The executable should be at `./target/release/midi2key.exe`.

## Usage

Run the executable file in an interactive terminal (or just double-click on Windows if you don't need to specify args).

Current command-line arguments available:

```plaintext
midi2key 0.2.0
Seercat3160
Bind MIDI notes to keyboard keys and mouse movement

USAGE:
    midi2key.exe [OPTIONS] --config <CONFIG>

OPTIONS:
    -c, --config <CONFIG>    Config file location
    -h, --help               Print help information
    -v, --verbose            Verbose mode
    -V, --version            Print version information
```

You will be prompted to select which MIDI device to use if multiple are connected.

### Configuration

At the moment, there is no real functionality. The program just prints details about keys pressed.

Due to this, there is no configuration necessary. In the future, however, a config file will be required to setup the key bindings and some other options. The location of this file will be specified with the command line option `-c`. If no config file exists there, it will be created with a default.

## Development

First of all, thank you for considering contributing to this side project of mine.

To build the project, follow the instructions in [the section about building from source](#building-from-source), without the `--release` flag of course.

It's just a standard Cargo setup, nothing out of the ordinary. However, make sure you have [rustfmt](https://github.com/rust-lang/rustfmt) installed, and run it before commiting. Otherwise, the debug build Action (which is a pull request check) will fail on your code.
