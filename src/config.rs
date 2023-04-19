use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};

use crate::note::Note;

/// Persistent app data, intended to be stored in a file
#[derive(Serialize, Deserialize)]
pub struct Config {
    binds: Vec<Bind>,
}

impl Config {
    /// Create a new, empty data store
    pub fn new() -> Self {
        Config { binds: vec![] }
    }

    /// Returns a clone of the bind at the given index, if it exists
    pub fn get_bind(&self, idx: usize) -> anyhow::Result<Bind> {
        Ok(self
            .binds
            .get(idx)
            .ok_or(anyhow!("index out of bounds for binds"))?
            .clone())
    }

    /// Add a given bind, returning it's index
    pub fn add_bind(&mut self, bind: Bind) -> usize {
        self.binds.push(bind);
        self.binds.len() - 1
    }

    /// Create a new bind with a default value, returning it's index
    pub fn add_default_bind(&mut self) -> usize {
        self.add_bind(Bind::default())
    }

    /// Delete the bind at the given index, if it exists
    pub fn delete_bind(&mut self, idx: usize) -> anyhow::Result<()> {
        // Check bounds
        if idx >= self.binds.len() {
            bail!("index out of bounds for binds");
        }

        self.binds.remove(idx);

        Ok(())
    }

    /// Returns clones of all binds for the given note
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

    /// Returns the note for a bind as a human-readable string, if it exists
    pub fn get_bind_note_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(self.get_bind(idx)?.note.into())
    }

    /// Returns a textual description of the action of a bind, if it exists
    pub fn get_bind_action_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(self.get_bind(idx)?.action.name())
    }

    /// Returns the current number of binds
    pub fn len_binds(&self) -> usize {
        self.binds.len()
    }

    /// Set the given index, if in bounds, to the given bind
    pub fn set_bind(&mut self, idx: usize, bind: Bind) -> anyhow::Result<()> {
        let idx_bind = self
            .binds
            .get_mut(idx)
            .ok_or(anyhow!("index out of bounds for binds"))?;

        *idx_bind = bind;

        Ok(())
    }
}

/// A mapping of MIDI-note to action
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Bind {
    /// MIDI note number
    pub note: Note,
    /// What should be done when the note is played
    pub action: BindAction,
}

/// Action taken when a Bind is executed
#[derive(Clone, Serialize, Deserialize)]
pub enum BindAction {
    PressKey(KeyboardKeyBindAction),
    HoldKey(KeyboardKeyBindAction),
    Click(MouseButton),
    HoldClick(MouseButton),
    MoveMouseRelative(RelativePos2D),
    MoveMouseAbsolute(AbsolutePos2D),
    Scroll(ScrollBindAction),
    Debug,
}

impl Default for BindAction {
    fn default() -> Self {
        Self::MoveMouseRelative(RelativePos2D::default())
    }
}

impl BindAction {
    /// Numerical representation of the enum
    pub fn index(&self) -> u8 {
        match self {
            BindAction::PressKey(_) => 0,
            BindAction::HoldKey(_) => 1,
            BindAction::Click(_) => 2,
            BindAction::HoldClick(_) => 3,
            BindAction::MoveMouseRelative(_) => 4,
            BindAction::MoveMouseAbsolute(_) => 5,
            BindAction::Scroll(_) => 6,
            BindAction::Debug => 7,
        }
    }

    /// String representation of the enum
    pub fn name(&self) -> String {
        match self {
            BindAction::PressKey(_) => "Press Key",
            BindAction::HoldKey(_) => "Hold Key",
            BindAction::Click(_) => "Click",
            BindAction::HoldClick(_) => "Hold Click",
            BindAction::MoveMouseRelative(_) => "Move Mouse",
            BindAction::MoveMouseAbsolute(_) => "Move Mouse to",
            BindAction::Scroll(_) => "Scroll",
            BindAction::Debug => "Debug",
        }
        .into()
    }
}

/// Data for an Action simulating a keypress
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct KeyboardKeyBindAction {
    pub key: String,
}

/// Data for an Action simulating a mouse click
#[derive(Clone, Default, Serialize, Deserialize)]
pub enum MouseButton {
    #[default]
    Left,
    Right,
    Middle,
}

impl MouseButton {
    /// Numerical representation of the enum
    pub fn index(&self) -> u8 {
        match self {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
        }
    }
}
/// Data for an Action changing a 2D position to a relative offset
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RelativePos2D {
    pub x: i32,
    pub y: i32,
}

/// Data for an Action changing a 2D position to an absolute value
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct AbsolutePos2D {
    pub x: i32,
    pub y: i32,
}

/// Data for an Action simulating mouse scroll
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ScrollBindAction {
    pub direction: ScrollDirection,
    pub amount: i32,
}

/// Cardinal screen direction
#[derive(Clone, Default, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl ScrollDirection {
    /// Numerical representation of the enum
    pub fn index(&self) -> u8 {
        match self {
            ScrollDirection::Up => 0,
            ScrollDirection::Down => 1,
            ScrollDirection::Left => 2,
            ScrollDirection::Right => 3,
        }
    }
}
