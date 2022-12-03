// Code from github.com/seercat3160/midi2key, under the MIT license

use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use midly::{live::LiveEvent, MidiMessage};

use clap::Parser;

use log::{error, info, warn};

fn main() {
    let _args = Args::parse();

    simple_logger::SimpleLogger::new().init().unwrap();

    let config: Midi2keyConfig =
        penguin_config::Deserializer::file_path("config.json").deserialize();

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
            warn!(
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
                            info!("hit note {} with vel {}", key, vel);

                            // Check if key bound in config, and if so execute any bindings
                            if config.bindings.contains_key(&key) {
                                match config.bindings.get(&key) {
                                    Some(i) => {
                                        for binding in i {
                                            let binding_name = binding.0;
                                            let binding_args = binding.1;

                                            invoke_binding(binding_name, binding_args, true, &vel, &key)
                                        }
                                    },
                                    None => error!("weird state that shouldn't be possible has been reached - type 1")
                                }
                            }
                        };
                    }
                    MidiMessage::NoteOff { key, vel: _ } => {
                        {
                            let key = key.as_int();
                            info!("released note {}", key);

                            // Check if key bound in config, and if so execute any bindings
                            if config.bindings.contains_key(&key) {
                                match config.bindings.get(&key) {
                                    Some(i) => {
                                        for binding in i {
                                            let binding_name = binding.0;
                                            let binding_args = binding.1;

                                            invoke_binding(binding_name, binding_args, false, &0, &key)
                                        }
                                    },
                                    None => error!("weird state that shouldn't be possible has been reached - type 2")
                                }
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

fn invoke_binding(binding: &str, args: &Vec<String>, state: bool, vel: &u8, key: &u8) {
    // Binding is the string for the binding (verbatim from the config)
    // Args is the binding's args from the config
    // State = true -> note hit
    // State = false -> note release
    // Vel is the note's velocity, or 0 for a release

    match state {
        true => match binding {
            "trace" => {
                warn!("Trace binding hit for key {} during note hit!", key)
            }
            _ => {
                error!(
                    "Config contains non-implemented binding {} in key {}",
                    binding, key
                );
            }
        },
        false => match binding {
            "trace" => {
                warn!("Trace binding hit for key {} during note release!", key)
            }
            _ => {
                error!(
                    "Config contains non-implemented binding {} in key {}",
                    binding, key
                );
            }
        },
    }
}
// Parse arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Verbose mode
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
}

// Config file
#[derive(penguin_config::Deserialize)]
struct Midi2keyConfig {
    bindings: HashMap<u8, HashMap<String, Vec<String>>>,
}
