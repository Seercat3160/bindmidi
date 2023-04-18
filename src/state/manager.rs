use std::sync::{mpsc::Receiver, Arc};

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
}

impl StateManager {
    pub fn new(state: State) -> (Self, Arc<StateInterface>) {
        let (state_interface, recv_channel) = StateInterface::new();

        (
            Self {
                state,
                channel: recv_channel,
            },
            state_interface,
        )
    }

    pub fn manage(&mut self) {
        while let Ok(message) = self.channel.recv() {
            match message.request {
                req::LenBinds => {
                    message
                        .response_channel
                        .send(res::LenBinds(self.state.config.len_binds()))
                        .expect("couldn't send");
                }
                req::NoteString(x) => {
                    message
                        .response_channel
                        .send(res::NoteString(self.state.config.get_bind_note_string(x)))
                        .expect("couldn't send");
                }
                req::ActionString(x) => {
                    message
                        .response_channel
                        .send(res::ActionString(
                            self.state.config.get_bind_action_string(x),
                        ))
                        .expect("couldn't send");
                }
                req::SetActiveBind(x) => {
                    self.state.set_active_bind(x);
                    message
                        .response_channel
                        .send(res::SetActiveBind)
                        .expect("couldn't send");
                }
                req::HasActiveBind => {
                    message
                        .response_channel
                        .send(res::HasActiveBind(self.state.has_active_bind()))
                        .expect("couldn't send");
                }
                req::GetActiveBind => {
                    message
                        .response_channel
                        .send(res::GetActiveBind(self.state.get_active_bind()))
                        .expect("couldn't send");
                }
                req::AddDefaultBind => {
                    message
                        .response_channel
                        .send(res::AddDefaultBind(self.state.config.add_default_bind()))
                        .expect("couldn't send");
                }
                req::DeleteActiveBind => {
                    message
                        .response_channel
                        .send(res::DeleteActiveBind(self.state.delete_active_bind()))
                        .expect("couldn't send");
                }
                req::UpdateActiveBind(x) => {
                    message
                        .response_channel
                        .send(res::UpdateActiveBind(self.state.update_active_bind(x)))
                        .expect("couldn't send");
                }
            };
        }
    }
}
