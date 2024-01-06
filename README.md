# midi2key

Conveniently bind MIDI notes to keyboard keys, mouse movement, and more!

midi2key is a program in Rust which provides a native GUI for the management and use of bindings from MIDI notes to actions taken on the system.

It has been tested on Windows and Linux (X11), and should theoretically work on MacOS as well. Linux Wayland support is not currently available, but planned.

[![Build Status](https://github.com/Seercat3160/midi2key/actions/workflows/build.yml/badge.svg)](https://github.com/Seercat3160/midi2key/actions/workflows/build.yml)

## Features

- [x] Keyboard actions
- [x] Mouse click
- [x] Mouse movement
- [x] Scrolling
- [x] Typing arbitrary strings
- [x] GUI for configuration and monitoring
- [ ] CLI usage
- [ ] Continuous mouse movement or scrolling while holding note
- [ ] Wayland support

## Installation

### Pre-built binaries

Visit [GitHub Releases](https://github.com/Seercat3160/midi2key/releases) to download the latest release binary for Linux and Windows. Alternatively, development builds can be acquired from GitHub Actions.

On Linux, it may be necessary to install `libxdo-dev` as a runtime dependency. It is packaged on Debian-based distros as `libxdo-dev`, and on Arch in `xdotool`.

On Windows, [Microsoft Visual C++ Redistributable](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170#visual-studio-2015-2017-2019-and-2022) must be installed - it is not (yet) shipped with the program. However, you probably already have it on your machine. Try launching the program and check out the link if there's errors about missing DLLs.

### Building from source

Simply clone this repository, ensure you have the latest Rust stable toolchain installed, and build with Cargo. I've found that some additional packages may be needed, such as the following Debian packages: `libasound2-dev libxdo-dev libgtk-3-dev libclang-dev`.

If you are intending to contribute code to this repo, thank you! In order for your PR to pass CI, you'll need to ensure it's fully formatted with `cargo fmt` and there are no warnings from `cargo clippy`. Ensure you have the latest stable toolchain installed, as that's what is used in CI, and sometimes there are additions and changes to clippy and format rules.

## Usage

Run the executable file. It will open the GUI.

To use the program, select your desired MIDI device in the drop-down menu in the left pane, and use the Start button. To stop, use the stop button. The current status of the program is available at the top of the left pane.

In contrast to previous versions of midi2key, no command-line interface is available, although this is being worked on.

## Configuration

All bindings (mappings from a MIDI note to an action performed) are configured through the right pane of the GUI. The table shows all existing bindings, and by default it will be empty. Binding edits made while the system is running will update as soon as they are saved, there is no need to stop and start again.

Using the buttons, one can create a new binding. Selecting a binding in the table shows buttons in the bottom-left of the window to edit the MIDI note which activates the binding, what action is performed, and any action-specific values.

After editing a binding in the GUI, ensure the "Save" button is used to apply the changes and write them to the persistent configuration file, located as follows:

- Linux: `$XDG_CONFIG_HOME/midi2key/config.json` or `$HOME/.config/midi2key/config.json`
- Windows: `%APPDATA%\midi2key\config\config.json`
- MacOS: `$HOME/Library/Application Support/midi2key/config.json`

### Available binding types

What follows is a brief description of the purpose of each available binding action. Hopefully the purpose of their arguments in the GUI is self-explanatory.

- **Press Key:** Simulates a press and release of a keyboard key when the MIDI note begins.
- **Hold Key:** Simulates the key being pressed down when the MIDI note starts, and released when the MIDI note ends.
- **Click:** Simulates the press and release of a mouse button when the MIDI note begins.
- **Hold Click:** Simulates the mouse button being pressed down when the MIDI note starts, and released when the MIDI note ends.
- **Move Mouse:** Moves the mouse a relative amount of pixels from its current position.
- **Move Mouse to:** Moves the mouse to an absolute position on the screen.
- **Scroll:** Scrolls the mouse a certain number of lines in the given direction.
- **Text:** Simulates typing of an arbitrary string of text.
- **Debug:** Prints a debug message to the console output. As an end user, ignore this.

### License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
