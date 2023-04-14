# midi2key

⚠️ **On this branch I'm currently rewriting most of the codebase to allow for better improvements in future and to add GUI support. Don't expect the rest of this README to be correct for this branch.** ⚠️

Bind MIDI notes to keyboard keys and mouse movement.

Midi2key is a simple program, written in Rust, to allow MIDI notes to trigger typing of keys, strings and movement of the mouse and scroll wheel.

It has been tested on Windows and Linux (X11), and should theoretically work on MacOS as well. Linux Wayland support is not currently possible, but planned.

[![Build Status](https://github.com/Seercat3160/midi2key/actions/workflows/build.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/build.yml)

## Features

- [x] Keyboard actions
- [x] Mouse click
- [x] Mouse movement
- [x] Scrolling
- [ ] Typing arbitrary strings
- [ ] Continuous mouse movement or scrolling while holding note
- [ ] GUI for configuration and monitoring
- [ ] Wayland support

## Installation

### Pre-built binaries

Visit [GitHub Releases](https://github.com/Seercat3160/midi2key/releases) to download the latest release binary for Windows and Linux.

## Usage

Run the executable file in an interactive terminal (or just double-click on the .exe in Windows if you don't need to specify arguments).

Current command-line arguments available:

```plaintext
Bind MIDI notes to keyboard keys and mouse movement

Usage: midi2key [OPTIONS]

Options:
  -v, --verbose        Verbose mode
  -c, --config <FILE>  Config file location [default: config.yml]
  -h, --help           Print help information
  -V, --version        Print version information
```

You will be prompted to select which MIDI device to use if multiple are connected.

## Configuration

To run the program, a config file must be present at either "config.yml" in the directory which the executable is being run from, or at the path specified by the command line option `--config`. If this file is not present, it will be created and pre-filled with the contents of `config.default.yml` in this repository.

To add bindings to the config, follow the example of the default config. Specify the note in the `bindings` map using [Scientific Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation), and then give an array of objects where the type of binding is given as `bind` and any arguments for that binding are also given. For all available bindings and their arguments, see below. Note that in the note names sharps must be typed as `#` and flats as `b`, and that bindings will be executed in the order they are specified.

### Available binding types

- `Trace`
  - Prints a debug message
  - **Arguments**: none
- `PressKey`
  - Simulates tapping a keyboard key
  - **Arguments**:
    - `key`: key to press
- `HoldKey`
  - Holds a keyboard key down as the note is held
  - **Arguments**:
    - `key`: key to hold
- `Click`
  - Simulates a mouse click
  - **Arguments**:
    - `button`: "Left" or "Right" to indicate which mouse button
- `HoldMouse`
  - Holds a mouse button as the note is held
  - **Arguments**:
    - `button`: "Left" or "Right" to indicate which mouse button
- `MoveMouse`
  - Moves the mouse relative to it's current position
  - **Arguments**:
    - `x`: pixels in the x direction (left and right) - Optional, defaults to 0
    - `y`: pixels in the y direction (up and down) - Optional, defaults to 0
- `MoveMouseAbsolute`
  - Moves the mouse to an absolute position on the screen (0,0 is the top left)
  - **Arguments**:
    - `x`: pixels in the x direction (left and right)
    - `y`: pixels in the y direction (up and down)
- `Scroll`
  - Scrolls the mouse wheel
  - **Arguments**:
    - `x`: lines to scroll in the x direction (left and right) - Optional, defaults to 0
    - `y`: lines to scroll in the y direction (up and down) - Optional, defaults to 0

## Development

First of all, thank you for considering contributing to this side project of mine.

Before committing, run [rustfmt](https://github.com/rust-lang/rustfmt) and [Clippy](https://github.com/rust-lang/rust-clippy) to ensure correct code style and practices. These checks are run in CI, so if anything's failing with them, the PR can't be merged. NB: Use the latest stable toolchain.

To test the software without connecting a real MIDI device, [VMPK](https://sourceforge.net/projects/vmpk/) can be used.
