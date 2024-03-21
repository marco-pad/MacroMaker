use crate::actions::Action;
use firmware::State;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Button {
    pub action: Action,
    pub state: State,
}
impl Default for Button {
    fn default() -> Self {
        Self {
            action: Action::Nothing,
            state: State::Released,
        }
    }
}

impl Button {
    pub const NOTHING: Self = Self {
        action: Action::Nothing,
        state: State::Released,
    };
}
