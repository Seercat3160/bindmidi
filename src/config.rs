use anyhow::anyhow;

use crate::note::Note;

pub(crate) struct Config {
    binds: Vec<Bind>,
}

impl Config {
    pub fn new() -> Self {
        Config { binds: vec![] }
    }

    #[allow(unused)] // Will be used in future, when we add the actual program functionality
    pub fn get_binds_for_note(&self, note: &Note) -> Vec<Bind> {
        self.binds
            .iter()
            .filter_map(|x| {
                if x.note == *note {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn set_binds(&mut self, binds: Vec<Bind>) {
        self.binds = binds;
    }

    pub fn get_note_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(self
            .binds
            .get(idx)
            .ok_or(anyhow!("index out of bounds for binds"))?
            .note
            .clone()
            .into())
    }

    pub fn get_nice_action_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(match self
            .binds
            .get(idx)
            .ok_or(anyhow!("index out of bounds for binds"))?
            .action
        {
            BindAction::PressKey(_) => "Press Key",
            BindAction::HoldKey(_) => "Hold Key",
            BindAction::Click(_) => "Click",
            BindAction::HoldClick(_) => "Hold Click",
            BindAction::MoveMouseRelative(_) => "Move Mouse",
            BindAction::MoveMouseAbsolute(_) => "Move Mouse to",
            BindAction::Scroll(_) => "Scroll",
        }
        .into())
    }

    pub fn len_binds(&self) -> usize {
        self.binds.len()
    }
}

/// A mapping of MIDI-note to action
#[derive(Clone, Default)]
pub struct Bind {
    /// MIDI note number
    pub note: Note,
    /// What should be done when the note is played
    pub action: BindAction,
}

/// Action taken when a Bind is executed
#[derive(Clone)]
#[allow(unused)]
pub enum BindAction {
    PressKey(KeyboardKeyBindAction),
    HoldKey(KeyboardKeyBindAction),
    Click(MouseButton),
    HoldClick(MouseButton),
    MoveMouseRelative(RelativePos2D),
    MoveMouseAbsolute(AbsolutePos2D),
    Scroll(ScrollBindAction),
}

impl Default for BindAction {
    fn default() -> Self {
        Self::MoveMouseRelative(RelativePos2D::default())
    }
}

/// Data for an Action simulating a keypress
#[derive(Clone, Default)]
#[allow(unused)]
pub struct KeyboardKeyBindAction {
    key: String,
}

/// Data for an Action simulating a mouse click
#[derive(Clone, Default)]
#[allow(unused)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

/// Data for an Action changing a 2D position to a relative offset
#[derive(Clone, Default)]
pub struct RelativePos2D {
    pub x: i32,
    pub y: i32,
}

/// Data for an Action changing a 2D position to an absolute non-negative value
#[derive(Clone, Default)]
#[allow(unused)]
pub struct AbsolutePos2D {
    x: u32,
    y: u32,
}

/// Data for an Action simulating mouse scroll
#[derive(Clone, Default)]
#[allow(unused)]
pub struct ScrollBindAction {
    amount: i32,
    direction: ScrollDirection,
}

/// Cardinal screen direction
#[derive(Clone, Default)]
#[allow(unused)]
enum ScrollDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}
