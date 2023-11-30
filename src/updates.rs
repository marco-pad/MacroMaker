use std::time::Instant;

use crate::{App, Edited, Menu, Message};
use iced::{Command, Theme};

pub fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        Message::FontLoaded(_) => (),
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
        Message::EditChanged => {
            app.edited = if app.edited == Edited::Macros {
                Edited::Rgb
            } else {
                Edited::Macros
            };
            app.when_edited = Instant::now();
        }
    };
    Command::none()
}
