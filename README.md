# midi2key

Bind MIDI notes to keyboard keys and mouse movement.

Midi2key is a simple program, written in Rust, to allow MIDI notes to trigger typing of keys, strings and movement of the mouse and scroll wheel.

[![Debug Build](https://github.com/Seercat3160/midi2key/actions/workflows/rust-debug.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/rust-debug.yml)
[![Release Build](https://github.com/Seercat3160/midi2key/actions/workflows/rust-release.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/rust-release.yml)

## Installation

### Pre-built binaries

You can visit [GitHub Releases](https://github.com/Seercat3160/midi2key/releases) to download the latest release binary. These should be ready to use (although make sure to check the description of the release for any known quirks).

If you want to use the latest version available, visit [Actions](https://github.com/Seercat3160/midi2key/actions) to download a binary for any commit. These will almost certainly be a work-in-progress with features half-baked, so steer clear unless you know it's what you need.

## Usage

Run the executable file in an interactive terminal (or just double-click on Windows if you don't need to specify args).

Current command-line arguments available:

```plaintext
Bind MIDI notes to keyboard keys and mouse movement

Usage: midi2key [OPTIONS]

Options:
  -v, --verbose        Verbose mode
  -c, --config <FILE>  Config file location [default: config.json]
  -h, --help           Print help information
  -V, --version        Print version information
```

You will be prompted to select which MIDI device to use if multiple are connected.

## Configuration

To run the program, a config file must be present at either "config.json" in the directory which the executable is being run from, or at the path specified by the command line option `--config`. If this file is not present, it will be created and pre-filled with the contents of `config.default.json` in this repository.

### Available binding types

- `trace`
  - Prints a debug message
  - **Arguments**: none
- `kclick`
  - Taps a key
  - **Arguments**: key to click
- `khold`
  - Holds a key down as the note is held
  - **Arguments**: key to click
- `mclickl`
  - Performs a left click
  - **Arguments**: none
- `mclickr`
  - Performs a right click
  - **Arguments**: none
- `mholdl`
  - Holds left click down as the note is held
  - **Arguments**: none
- `mholdr`
  - Holds right click down as the note is held
  - **Arguments**: none
- `mmoverel`
  - Moves the mouse relative to it's current position
  - **Arguments**:
    - **Integer**: pixels in the `x` direction
    - **Integer**: pixels in the `y` direction
- `mscrolly`
  - Scrolls on the `y` axis - up and down
  - **Arguments**:
    - **Integer**: lines to scroll in the `y` direction
- `mscrollx`
  - Scrolls on the `x` axis - left and right
  - **Arguments**:
    - **Integer**: lines to scroll in the `x` direction

## Development

First of all, thank you for considering contributing to this side project of mine.

It's just a standard Cargo setup, nothing out of the ordinary. However, make sure you have [rustfmt](https://github.com/rust-lang/rustfmt) installed, and run it before commiting. Otherwise, the debug build Action (which is a pull request check) will fail on your code.

To test the software without connecting a real MIDI device, I use [VMPK](https://sourceforge.net/projects/vmpk/).
