use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Midi2keyConfig {
    pub(crate) verbose: bool,
    pub(crate) bindings: HashMap<u8, Vec<Binding>>,
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
    LEFT,
    RIGHT,
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
