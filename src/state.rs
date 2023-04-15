use std::sync::mpsc;

use crate::{config::Config, statechannel::StateChannelMessage};

struct State {
    config: Config,
}

impl State {
    fn new() -> Self {
        State {
            config: Config::new(),
        }
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

pub fn handler(state_channel_receiver: &mpsc::Receiver<StateChannelMessage>) {
    let mut state = State::new();

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
            StateChannelMessage::SetBinds: binds => {
                state.config.set_binds(binds)
            }
        });
    }
}
