use crate::actions::Action;
use crate::connection::Connection;
use crate::{select_button, selected_button, App, Menu, BUTTONS, CONNECTION};
use iced::{Command, Event, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    Nothing,
    FontLoaded(Result<(), super::font::Error>),
    Settings(bool),
    ThemeChanged(bool),
    Button(usize),
    Input,
    Event(Event),

    EditButton(Action),
    RecordKey,
    EditCommand(String),
}

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    let mut command = Command::none();
    match message {
        Message::Nothing => (),
        Message::FontLoaded(_) => (),
        Message::Input => {
            // command = Command::perform(
            //     async {
            //         unsafe {
            //             if let Some(connection) = CONNECTION.as_ref() {
            //                 connection.recv().await.unwrap();
            //             } else {
            //                 CONNECTION = Some(Connection::new().await.unwrap());
            //             }
            //         }
            //     },
            //     |_| Message::Input,
            // );
        }
        Message::Event(event) => {
            if let Event::Keyboard(iced::keyboard::Event::KeyPressed { key_code, .. }) = event {
                if app.recording {
                    BUTTONS.lock()[selected_button() - 1].action = Action::Keypress(key_code)
                }
                app.recording = false;
            }
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
