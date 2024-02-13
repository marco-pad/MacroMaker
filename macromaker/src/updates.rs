use crate::actions::Action;
use crate::{select_button, selected_button, App, Menu, BUTTONS};
use iced::{Command, Event, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    FontLoaded(Result<(), super::font::Error>),
    Settings(bool),
    ThemeChanged(bool),
    Button(usize),
    Event(Event),
    /// Whenever input is detected by the device.
    Input,

    EditButton(Action),
    RecordKey,
    EditCommand(String),
}

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    let command = Command::none();
    match message {
        Message::Nothing => (),
        Message::FontLoaded(_) => (),
        Message::Event(event) => {
            if let Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) = event {
                if app.recording {
                    BUTTONS.lock()[selected_button() - 1].action = Action::Keypress(key_code)
                }
                app.recording = false;
            }
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
