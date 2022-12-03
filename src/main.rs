// Code from github.com/seercat3160/midi2key, under the MIT license

use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput};
use midly::{live::LiveEvent, MidiMessage};

use clap::Parser;

use log::{info, warn};

fn main() {
    let _args = Args::parse();

    simple_logger::SimpleLogger::new().init().unwrap();

    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

// Setup midir and connect to devices
fn run() -> Result<(), Box<dyn Error>> {
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
            on_midi(message);
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

// Parse MIDI messages
fn on_midi(event: &[u8]) {
    let event = LiveEvent::parse(event).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                note_start(key.as_int(), vel.as_int());
            }
            MidiMessage::NoteOff { key, vel: _ } => {
                note_end(key.as_int());
            }
            _ => {}
        },
        _ => {}
    }
}

// Act on MIDI notes starting
fn note_start(key: u8, vel: u8) {
    info!("hit note {} with vel {}", key, vel);
}

// Act on MIDI notes ending
fn note_end(key: u8) {
    info!("released note {}", key);
}

// Parse arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Config file location
    #[clap(short, long, value_parser)]
    config: String,

    /// Verbose mode
    #[clap(short, long, value_parser, default_value_t = false)]
    verbose: bool,
}
