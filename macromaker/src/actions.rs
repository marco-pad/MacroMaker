/// Here every action gets executed
use std::time::Duration;

use enigo::{Enigo, KeyboardControllable};
use firmware::State;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

static ENIGO: Lazy<Mutex<Enigo>> = Lazy::new(|| Mutex::new(Enigo::new()));

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Action {
    #[default]
    Nothing,
    Keypress(String),
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
        Action::Keypress(String::new()),
        Action::Macro(Macro::new()),
        Action::Command(String::new()),
    ];
    pub fn perform(&self, state: State) {
        match self {
            Self::Keypress(string) => match state {
                State::Pressed => ENIGO.lock().key_sequence(string),
                State::Released => (),
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[allow(unused)]
enum MacroAction {
    Word(String),
    Command(String),
    Wait(Duration),
}
impl std::fmt::Display for MacroAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Word(_word) => {
                write!(f, "Word")
            }
            Self::Command(_) => {
                write!(f, "Terminal Command")
            }
            Self::Wait(_) => write!(f, "Wait"),
        }
    }
}

// fn iced_to_enigo_key(key: iced::keyboard::Key) {
//     //-> enigo::Key {
//     use enigo::Key;
//     use iced::keyboard::key::Named;
//     if let iced::keyboard::Key::Named(named_key) = key {
//         // match named_key {
//         //     Named::Alt => Key::Alt,
//         //     Named::CapsLock => Key::CapsLock,
//         //     Named::Control => Key::Control,
//         // }
//     } else {
//         // enigo::Key::Raw(3)
//     }
// }
