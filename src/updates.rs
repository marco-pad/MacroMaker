use crate::{App, Message, Menu, Edited};
use iced::{Command, Theme};

pub fn update<'a>(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::FontLoaded(_) => (),
        Message::Settings(val) => {
            app.menu = val.then_some(Menu::Settings).unwrap_or(Menu::Main);
        }
        Message::ThemeChanged(val) => {
            app.theme_light = val;
            app.theme = app
                .theme_light
                .then_some(Theme::Light)
                .unwrap_or(Theme::Dark);
        }
        Message::EditChanged => {
            app.edited = (app.edited == Edited::Macros).then_some(Edited::Rgb).unwrap_or(Edited::Macros);
        }
    };
    Command::none()
}