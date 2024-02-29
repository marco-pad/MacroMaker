use crate::actions::Action;
use crate::{select_button, selected_button, App, Menu, BUTTONS};
use iced::{Command, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    FontLoaded(Result<(), super::font::Error>),
    Settings(bool),
    ThemeChanged(bool),
    Button(usize),
    /// Whenever input is detected by the device.
    Input,

    EditButton(Action),
    RecordKey,
    EditCommand(String),
    KeyPress(iced::keyboard::Key),
}
pub fn update(app: &mut App, message: Message) -> Command<Message> {
    let command = Command::none();
    match message {
        Message::Nothing => (),
        Message::FontLoaded(_) => (),
        Message::KeyPress(key) => {
            if app.recording {
                BUTTONS.lock()[selected_button() - 1].action = Action::Keypress(key)
            }
            app.recording = false;
        }
        Message::Input => {
            println!("Input detected");
        }
        Message::Settings(val) => {
            app.menu = if val { Menu::Settings } else { Menu::Main };
        }
        Message::ThemeChanged(val) => {
            app.theme_light = val;
            app.theme = if app.theme_light {
                Theme::Light
            } else {
                Theme::Dark
            };
        }
        Message::Button(id) => {
            select_button(id);
        }
        Message::EditButton(action) => {
            BUTTONS.lock()[selected_button() - 1].action = action;
        }
        Message::RecordKey => {
            app.recording = true;
        }
        Message::EditCommand(command) => {
            BUTTONS.lock()[selected_button() - 1].action = Action::Command(command);
        }
    };
    command
}
