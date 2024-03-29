use std::sync::{mpsc::Receiver, Arc};

use crate::bind::Executor;

use super::{
    interface::{
        StateInterface, StateMessage, StateMessageRequest as req, StateMessageResponse as res,
    },
    State,
};

/// Holds the state and runs the main loop allowing other parts of the program to query/mutate the state safely through message passing
pub struct StateManager {
    /// Application state, held by this Manager
    state: State,
    /// Channel for this Manager to receive data from elsewhere
    channel: Receiver<StateMessage>,
    /// A clone of the sender side of this Manager's channel, so it can start other things which can send data back to it
    interface: Arc<StateInterface>,
}

impl StateManager {
    pub fn new(state: State) -> (Self, Arc<StateInterface>) {
        let (state_interface, recv_channel) = StateInterface::new();

        (
            Self {
                state,
                channel: recv_channel,
                interface: state_interface.clone(),
            },
            state_interface,
        )
    }

    pub fn manage(&mut self) -> anyhow::Result<()> {
        self.state.init_midi("bindmidi")?;

        while let Ok(message) = self.channel.recv() {
            match message.request {
                req::LenBinds => {
                    message
                        .response_channel
                        .send(res::LenBinds(self.state.config.len_binds()))?;
                }
                req::NoteString(x) => {
                    message
                        .response_channel
                        .send(res::NoteString(self.state.config.get_bind_note_string(x)))?;
                }
                req::ActionString(x) => {
                    message.response_channel.send(res::ActionString(
                        self.state.config.get_bind_action_string(x),
                    ))?;
                }
                req::SetActiveBind(x) => {
                    self.state.set_active_bind(x);
                    message.response_channel.send(res::SetActiveBind)?;
                }
                req::HasActiveBind => {
                    message
                        .response_channel
                        .send(res::HasActiveBind(self.state.has_active_bind()))?;
                }
                req::GetActiveBind => {
                    message
                        .response_channel
                        .send(res::GetActiveBind(self.state.get_active_bind()))?;
                }
                req::AddDefaultBind => {
                    message
                        .response_channel
                        .send(res::AddDefaultBind(self.state.config.add_default_bind()))?;
                }
                req::DeleteActiveBind => {
                    message
                        .response_channel
                        .send(res::DeleteActiveBind(self.state.delete_active_bind()))?;
                }
                req::UpdateActiveBind(x) => {
                    message
                        .response_channel
                        .send(res::UpdateActiveBind(self.state.update_active_bind(x)))?;
                }
                req::MidiInputNames => {
                    message
                        .response_channel
                        .send(res::MidiInputNames(self.state.get_midi_input_names()))?;
                }
                req::SetMidiInputPort(x) => {
                    self.state.set_midi_input_port(x)?;
                    message.response_channel.send(res::SetMidiInputPort)?;
                }
                req::StartMidiConnection => {
                    self.state
                        .start_midi_connection("bindmidi", self.interface.clone())?;
                    message.response_channel.send(res::StartMidiConnection)?;
                }
                req::StopMidiConnection => {
                    self.state.stop_midi_connection();
                    message.response_channel.send(res::StopMidiConnection)?;
                }
                req::HasMidiConnection => {
                    message
                        .response_channel
                        .send(res::HasMidiConnection(self.state.has_midi_connection()))?;
                }
                req::ExecuteBindsForNote(note, vel, state) => {
                    let binds = self.state.config.get_binds_for_note(&note);

                    let mut executor = Executor::new();

                    for bind in binds {
                        executor.execute(&bind, vel, &note, &state)?;
                    }

                    message.response_channel.send(res::ExecuteBindsForNote)?;
                }
                req::SaveConfig(path) => {
                    self.state.save_config(path)?;
                    message.response_channel.send(res::SaveConfig)?;
                }
                req::Shutdown => {
                    // Stop MIDI connection (if any)
                    self.state.stop_midi_connection();

                    message.response_channel.send(res::Shutdown)?;

                    // End this thread by returning
                    return Ok(());
                }
            };
        }

        Ok(())
    }
}
