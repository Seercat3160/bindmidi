// Code from github.com/seercat3160/midi2key, under the MIT license

mod config;
use config::{Binding, Midi2keyConfig, StubConfig};

use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::exit;

use enigo::{Enigo, Key, KeyboardControllable, MouseButton, MouseControllable};

use midir::{Ignore, MidiInput};
use midly::{live::LiveEvent, MidiMessage};

use clap::Parser;

use log::{error, info, warn};

static CONFIG_VERSION: u8 = 1;

#[cfg(test)]
mod test;

fn main() {
    // Parse program arguments
    let args = Args::parse();

    // Setup logging
    simple_logger::SimpleLogger::new().init().unwrap();

    // Check if config file exists
    let config_path = Path::new(&args.config);
    if !config_path.exists() {
        warn!("No config file was found at the specified location ({}), so a default config was placed there.", args.config);
        let mut output = File::create(config_path).expect("couldn't create default config file");
        write!(output, "{}", include_str!("../config.default.yml")).unwrap();
    }

    // Read config file to a string
    let config_file_contents = read_to_string(config_path).expect("Couldn't read config file!");

    // Check config file version against what is compatible
    if serde_yaml::from_str::<StubConfig>(&config_file_contents)
        .expect("Invalid config file!")
        .version
        != CONFIG_VERSION
    {
        error!("Config has unsupported version, this version of midi2key only supports config version {}!", CONFIG_VERSION);
        exit(1);
    }

    // Deserialize full config
    let mut config: Midi2keyConfig =
        serde_yaml::from_str(&config_file_contents).expect("Invalid config file!");

    // Program argument overrides the config file
    if args.verbose == true {
        config.verbose = true;
    }

    // Exit if there are no bindings setup in the config (and verbose mode isn't enabled as this can be used to figure out which key is which note number in order to write the config)
    if config.bindings.len() == 0 && config.verbose == false {
        error!("The current config file contains no bindings - exiting!");
        exit(1);
    }

    if config.verbose {
        info!("Config version: {}", config.version);
    }

    match run(config) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

// Setup midir and connect to devices
fn run(config: Midi2keyConfig) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            info!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message, _| {
            match LiveEvent::parse(message).unwrap() {
                LiveEvent::Midi {
                    channel: _,
                    message,
                } => match message {
                    MidiMessage::NoteOn { key, vel } => {
                        {
                            let key = key.as_int();
                            let vel = vel.as_int();
                            if config.verbose {
                                info!("hit note {} with vel {}", key, vel);
                            }

                            // Check if key bound in config, and if so execute any bindings
                            match config.bindings.get(&key) {
                                Some(i) => {
                                    for binding in i {
                                        invoke_binding(
                                            binding,
                                            BindingNoteState::NoteOn,
                                            &vel,
                                            &key,
                                        )
                                    }
                                }
                                None => {}
                            }
                        };
                    }
                    MidiMessage::NoteOff { key, vel } => {
                        {
                            let key = key.as_int();
                            let vel = vel.as_int();
                            if config.verbose {
                                info!("released note {} with vel {}", key, vel);
                            }

                            // Check if key bound in config, and if so execute any bindings
                            match config.bindings.get(&key) {
                                Some(i) => {
                                    for binding in i {
                                        invoke_binding(
                                            binding,
                                            BindingNoteState::NoteOff,
                                            &vel,
                                            &key,
                                        )
                                    }
                                }
                                None => {}
                            }
                        };
                    }
                    _ => {}
                },
                _ => {}
            };
        },
        (),
    )?;

    warn!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    warn!("Closing connection");
    Ok(())
}

fn invoke_binding(binding: &Binding, state: BindingNoteState, vel: &u8, key: &u8) {
    use BindingNoteState::{NoteOff, NoteOn};

    let mut enigo = Enigo::new();

    match binding {
        Binding::Trace => match state {
            NoteOn => warn!(
                "Trace binding hit during note start! Note: {}, Velocity: {}",
                key, vel
            ),
            NoteOff => warn!(
                "Trace binding hit during note release! Note: {}, Velocity: {}",
                key, vel
            ),
        },
        Binding::PressKey(b) => match state {
            NoteOn => {
                enigo.key_click(Key::Layout(b.key));
            }
            _ => {}
        },
        Binding::HoldKey(b) => match state {
            NoteOn => {
                enigo.key_down(Key::Layout(b.key));
            }
            NoteOff => enigo.key_up(Key::Layout(b.key)),
        },
        Binding::Click(b) => match state {
            NoteOn => match b.button {
                config::MouseButton::Left => {
                    enigo.mouse_click(MouseButton::Left);
                }
                config::MouseButton::Right => {
                    enigo.mouse_click(MouseButton::Right);
                }
            },
            _ => {}
        },
        Binding::HoldMouse(b) => match state {
            NoteOn => match b.button {
                config::MouseButton::Left => {
                    enigo.mouse_down(MouseButton::Left);
                }
                config::MouseButton::Right => {
                    enigo.mouse_down(MouseButton::Right);
                }
            },
            NoteOff => match b.button {
                config::MouseButton::Left => {
                    enigo.mouse_up(MouseButton::Left);
                }
                config::MouseButton::Right => {
                    enigo.mouse_up(MouseButton::Right);
                }
            },
        },
        Binding::MoveMouse(b) => match state {
            NoteOn => {
                enigo.mouse_move_relative(b.x, b.y);
            }
            _ => {}
        },
        Binding::Scroll(b) => match state {
            NoteOn => {
                enigo.mouse_scroll_x(b.x);
                enigo.mouse_scroll_y(b.y);
            }
            _ => {}
        },
    }
}

enum BindingNoteState {
    NoteOn,
    NoteOff,
}
// Parse arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Verbose mode - overrides the value from the config
    #[arg(short, long, value_parser, default_value_t = false)]
    verbose: bool,

    /// Config file location
    #[arg(short, long, value_name = "FILE", default_value_t = String::from("config.yml"))]
    config: String,
}
