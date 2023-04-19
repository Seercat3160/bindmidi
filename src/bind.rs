use anyhow::anyhow;
use enigo::{Enigo, KeyboardControllable, MouseControllable};

use crate::{config::Bind, note::Note};

/// Executes binds
pub struct Executor {
    runtime: Enigo,
}

impl Executor {
    /// Creates a new Executor using Enigo
    pub fn new() -> Self {
        Self {
            runtime: Enigo::default(),
        }
    }

    /// Execute a bind
    pub fn execute(
        &mut self,
        bind: &Bind,
        vel: u8,
        note: &Note,
        state: &BindExecuteState,
    ) -> anyhow::Result<()> {
        match &bind.action {
            crate::config::BindAction::Debug => {
                println!("Bind executed! state: {state:?}, note: {note}, velocity: {vel}");
            }
            crate::config::BindAction::PressKey(param) => {
                if let BindExecuteState::Begin = state {
                    self.runtime.key_click(enigo::Key::Layout(
                        param
                            .key
                            .chars()
                            .next()
                            .ok_or(anyhow!("invalid keyboard key in bind"))?,
                    ));
                }
            }
            crate::config::BindAction::HoldKey(param) => {
                let key = enigo::Key::Layout(
                    param
                        .key
                        .chars()
                        .next()
                        .ok_or(anyhow!("invalid keyboard key in bind"))?,
                );
                match state {
                    BindExecuteState::Begin => {
                        self.runtime.key_down(key);
                    }
                    BindExecuteState::Release => {
                        self.runtime.key_up(key);
                    }
                }
            }
            crate::config::BindAction::Click(param) => {
                if let BindExecuteState::Begin = state {
                    self.runtime.mouse_click(match param {
                        crate::config::MouseButton::Left => enigo::MouseButton::Left,
                        crate::config::MouseButton::Right => enigo::MouseButton::Right,
                        crate::config::MouseButton::Middle => enigo::MouseButton::Middle,
                    });
                }
            }
            crate::config::BindAction::HoldClick(param) => match state {
                BindExecuteState::Begin => {
                    self.runtime.mouse_down(match param {
                        crate::config::MouseButton::Left => enigo::MouseButton::Left,
                        crate::config::MouseButton::Right => enigo::MouseButton::Right,
                        crate::config::MouseButton::Middle => enigo::MouseButton::Middle,
                    });
                }
                BindExecuteState::Release => {
                    self.runtime.mouse_up(match param {
                        crate::config::MouseButton::Left => enigo::MouseButton::Left,
                        crate::config::MouseButton::Right => enigo::MouseButton::Right,
                        crate::config::MouseButton::Middle => enigo::MouseButton::Middle,
                    });
                }
            },
            crate::config::BindAction::MoveMouseRelative(param) => {
                if let BindExecuteState::Begin = state {
                    self.runtime.mouse_move_relative(param.x, param.y);
                }
            }
            crate::config::BindAction::MoveMouseAbsolute(param) => {
                if let BindExecuteState::Begin = state {
                    self.runtime.mouse_move_to(param.x, param.y);
                }
            }
            crate::config::BindAction::Scroll(param) => {
                if let BindExecuteState::Begin = state {
                    let scroll_x = match param.direction {
                        crate::config::ScrollDirection::Right => param.amount,
                        crate::config::ScrollDirection::Left => -param.amount,
                        _ => 0,
                    };
                    let scroll_y = match param.direction {
                        crate::config::ScrollDirection::Down => param.amount,
                        crate::config::ScrollDirection::Up => -param.amount,
                        _ => 0,
                    };

                    self.runtime.mouse_scroll_x(scroll_x);
                    self.runtime.mouse_scroll_y(scroll_y);
                }
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum BindExecuteState {
    Begin,
    Release,
}
