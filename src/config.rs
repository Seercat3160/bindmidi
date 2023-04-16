use anyhow::anyhow;

use crate::note::Note;

pub struct Config {
    binds: Vec<Bind>,
}

impl Config {
    pub fn new() -> Self {
        Config { binds: vec![] }
    }

    //TODO: Remove this when adding and editing binds from GUI is implemented
    pub fn new_with_prefilled_values_for_debug() -> Self {
        let mut config = Self::new();

        config.add_bind(Bind::default());
        config.add_bind(Bind {
            note: Note::default(),
            action: BindAction::Click(MouseButton::Left),
        });
        config.add_bind(Bind {
            note: Note::from_midi(73),
            action: BindAction::Click(MouseButton::Right),
        });

        config
    }

    /// Returns a clone of the bind at the given index, if it exists
    pub fn get_bind(&self, idx: usize) -> anyhow::Result<Bind> {
        Ok(self
            .binds
            .get(idx)
            .ok_or(anyhow!("index out of bounds for binds"))?
            .clone())
    }

    /// Add a bind, returning it's index
    pub fn add_bind(&mut self, bind: Bind) -> usize {
        self.binds.push(bind);
        self.binds.len() - 1
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

    pub fn get_note_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(self.get_bind(idx)?.note.into())
    }

    pub fn get_nice_action_string(&self, idx: usize) -> anyhow::Result<String> {
        Ok(self.get_bind(idx)?.action.name())
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
        }
        .into()
    }
}

/// Data for an Action simulating a keypress
#[derive(Clone, Default)]
pub struct KeyboardKeyBindAction {
    pub key: String,
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
#[derive(Clone, Default)]
pub struct RelativePos2D {
    pub x: i32,
    pub y: i32,
}

/// Data for an Action changing a 2D position to an absolute value
#[derive(Clone, Default)]
pub struct AbsolutePos2D {
    pub x: i32,
    pub y: i32,
}

/// Data for an Action simulating mouse scroll
#[derive(Clone, Default)]
pub struct ScrollBindAction {
    pub direction: ScrollDirection,
    pub amount: i32,
}

/// Cardinal screen direction
#[derive(Clone, Default)]
#[allow(unused)]
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
