use std::collections::HashMap;

use musical_scales::Pitch;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Midi2keyConfig {
    pub(crate) version: u8,
    pub(crate) verbose: bool,
    pub(crate) bindings: HashMap<Pitch, Vec<Binding>>,
}

#[derive(Deserialize)]
pub(crate) struct StubConfig {
    pub(crate) version: u8,
}

#[derive(Deserialize)]
#[serde(tag = "bind")]
pub(crate) enum Binding {
    Trace,
    PressKey(PressKeyBinding),
    HoldKey(HoldKeyBinding),
    Click(ClickBinding),
    HoldMouse(HoldMouseBinding),
    MoveMouse(MoveMouseBinding),
    Scroll(ScrollBinding),
}

#[derive(Deserialize)]
pub(crate) struct PressKeyBinding {
    pub(crate) key: char,
}

#[derive(Deserialize)]
pub(crate) struct HoldKeyBinding {
    pub(crate) key: char,
}

#[derive(Deserialize)]
pub(crate) enum MouseButton {
    Left,
    Right,
}

#[derive(Deserialize)]
pub(crate) struct ClickBinding {
    pub(crate) button: MouseButton,
}

#[derive(Deserialize)]
pub(crate) struct HoldMouseBinding {
    pub(crate) button: MouseButton,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct MoveMouseBinding {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct ScrollBinding {
    pub(crate) x: i32,
    pub(crate) y: i32,
}
