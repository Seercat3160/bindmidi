use std::sync::Arc;

use anyhow::bail;
use midir::{MidiInput, MidiInputConnection};
use midly::live::LiveEvent;

use crate::note::Note;

use super::interface::StateInterface;

#[derive(Default)]
pub struct Midi {
    /// MIDI input, potentially not yet initialised
    input: Option<MidiInput>,
    /// MIDI input port to use when starting the MIDI connection
    port: Option<usize>,
    /// Known port names - This is used to cache them so we don't have an input and a connection simultaneously
    port_names: Vec<String>,
    /// Possible MIDI connection
    connection: Option<MidiInputConnection<Arc<StateInterface>>>,
    /// Is there an open connection?
    pub has_open_connection: bool,
}

impl Midi {
    /// Start a new MIDI connection
    pub fn start_midi_connection(
        &mut self,
        conn_name: &str,
        state_interface: Arc<StateInterface>,
    ) -> anyhow::Result<()> {
        self.init_midi(conn_name)?;

        let input = self.input.take().expect("no midi input");

        let ports = input.ports();
        let port = ports
            .get(self.port.unwrap_or(0))
            .expect("no midi input ports");

        let connection = input
            .connect(port, conn_name, handle_midi_message, state_interface)
            .expect("couldn't connect");
        self.connection.replace(connection);

        self.has_open_connection = true;

        Ok(())
    }

    /// Close the current MIDI connection, if there is one
    pub fn stop_midi_connection(&mut self) {
        // Safe to stop if there's not a connection running
        if !self.has_open_connection {
            return;
        }

        let connection = self.connection.take().expect("no open connection");
        self.has_open_connection = false;

        let (input, _) = connection.close();

        // Again use the returned Input as ours
        self.input.replace(input);
    }

    /// Setup Midi input
    pub fn init_midi(&mut self, client_name: &str) -> anyhow::Result<()> {
        // Stop if there's an open connection currently
        if self.has_open_connection {
            bail!("Can't init MIDI, there's an open connection already")
        }

        self.input.replace(MidiInput::new(client_name)?);

        self.connection.take();
        self.has_open_connection = false;

        Ok(())
    }

    /// Get the names of all available Midi input ports
    pub fn get_midi_input_names(&mut self) -> anyhow::Result<Vec<String>> {
        // If there's a connection open, early return the cached names
        if self.has_open_connection {
            return Ok(self.port_names.clone());
        }

        // Get new data if we can
        if let Some(input) = &self.input {
            let mut names = vec![];

            for port in input.ports() {
                names.push(input.port_name(&port)?);
            }

            self.port_names = names;
        }

        Ok(self.port_names.clone())
    }

    /// Set the stored Midi input port index to the given index
    pub fn set_midi_input_port(&mut self, idx: usize) -> anyhow::Result<()> {
        let ports = self.get_midi_input_names()?;

        if idx < ports.len() {
            self.port = Some(idx);
        } else {
            bail!("given port index out of range")
        }

        Ok(())
    }
}

fn handle_midi_message(
    _timestamp: u64,
    midi_data: &[u8],
    state_interface: &mut Arc<StateInterface>,
) {
    let event = LiveEvent::parse(midi_data).unwrap();
    if let LiveEvent::Midi {
        channel: _,
        message,
    } = event
    {
        match message {
            midly::MidiMessage::NoteOff { key, vel } => {
                let note = Note::from_midi(key.as_int());

                state_interface.execute_binds(
                    note,
                    vel.as_int(),
                    crate::bind::BindExecuteState::Begin,
                );
            }
            midly::MidiMessage::NoteOn { key, vel } => {
                let note = Note::from_midi(key.as_int());

                state_interface.execute_binds(
                    note,
                    vel.as_int(),
                    crate::bind::BindExecuteState::Release,
                );
            }
            _ => (),
        }
    }
}
