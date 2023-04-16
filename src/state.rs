use std::sync::mpsc;

use crate::{
    config::{Bind, Config},
    statechannel::StateChannelMessage,
};

pub struct State {
    config: Config,
    /// Index of the bind currently being edited in the GUI
    active_edit_bind: Option<usize>,
}

impl State {
    pub fn with_config(config: Config) -> Self {
        State {
            config,
            active_edit_bind: None,
        }
    }

    /// Updates which bind is currently being edited in the GUI, so
    /// we can access it more easily without first having to know anything about it
    fn set_active_edit_bind(&mut self, idx: Option<usize>) {
        self.active_edit_bind = idx;
    }

    /// Returns true if there is an active edit bind, false if not
    fn has_active_edit_bind(&self) -> bool {
        self.active_edit_bind.is_some()
    }

    /// Returns a clone of the current active edit bind, if there is one
    fn get_active_edit_bind(&self) -> Option<Bind> {
        self.active_edit_bind.map(|idx| {
            self.config
                .get_bind(idx)
                .expect("active edit bind shouldn't be out-of-bounds")
        })
    }
}

macro_rules! handle {
    (
        $message:ident {
            $(
                $which:path$(: $($param:ident) +)? => $block:block
            ),+
        }
    ) => {
        match $message {
            $(
                $which (tx$(, $($param),*)?) => {
                    #[allow(clippy::semicolon_if_nothing_returned)]
                    tx.send($block).unwrap();
                }
            ),+
        }
    };
}

pub fn handler(state_channel_receiver: &mpsc::Receiver<StateChannelMessage>, mut state: State) {
    while let Ok(message) = state_channel_receiver.recv() {
        handle!(message {
            StateChannelMessage::LenBinds => {
                state.config.len_binds()
            },
            StateChannelMessage::NoteString: idx => {
                state.config.get_note_string(idx)
            },
            StateChannelMessage::ActionString: idx => {
                state.config.get_nice_action_string(idx)
            },
            StateChannelMessage::SetActiveEditBind: idx => {
                state.set_active_edit_bind(idx)
            },
            StateChannelMessage::HasActiveEditBind => {
                state.has_active_edit_bind()
            },
            StateChannelMessage::GetActiveEditBind => {
                state.get_active_edit_bind()
            }
        });
    }
}
