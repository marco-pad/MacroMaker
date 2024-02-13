/// Here every action gets executed
use std::time::Duration;

use enigo::{Enigo, KeyboardControllable};
use firmware::State;
use iced::keyboard::KeyCode;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

static ENIGO: Lazy<Mutex<Enigo>> = Lazy::new(|| Mutex::new(Enigo::new()));

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Action {
    #[default]
    Nothing,
    Keypress(KeyCode),
    Macro(Macro),
    Command(String),
}
impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nothing => {
                write!(f, "nothing")
            }
            Self::Keypress(_) => {
                write!(f, "keyboard keypress")
            }
            Self::Macro(_) => {
                write!(f, "macro")
            }
            Self::Command(_) => {
                write!(f, "terminal command")
            }
        }
    }
}
impl Action {
    pub const ALL: [Action; 4] = [
        Action::Nothing,
        Action::Keypress(KeyCode::E),
        Action::Macro(Macro::new()),
        Action::Command(String::new()),
    ];
    pub fn perform(&self, state: State) {
        match self {
            Self::Keypress(key) => match state {
                State::Pressed => ENIGO.lock().key_down(enigo::Key::Raw(*key as u16)),
                State::Released => ENIGO.lock().key_up(enigo::Key::Raw(*key as u16)),
            },
            Self::Macro(_combination) => {
                todo!("run macro");
            }
            Self::Command(command) => {
                if state == State::Pressed {
                    let _ = std::process::Command::new(command)
                        .status()
                        .inspect_err(|e| {
                            dbg!(e);
                        });
                }
            }
            Self::Nothing => (),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Macro(Vec<MacroAction>);
impl Macro {
    pub const fn new() -> Self {
        Self(vec![])
    }
}
impl Default for Macro {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(unused)]
enum MacroAction {
    Keypress(KeyCode),
    Command(String),
    Wait(Duration),
}
impl std::fmt::Display for MacroAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keypress(_key) => {
                write!(f, "Keypress")
            }
            Self::Command(_) => {
                write!(f, "Terminal Command")
            }
            Self::Wait(_) => write!(f, "Wait"),
        }
    }
}
