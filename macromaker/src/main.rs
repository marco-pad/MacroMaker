#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

use iced::{executor, font, window, Font};
use iced::{Application, Command, Renderer, Settings, Theme};
#[cfg(target_os = "windows")]
use resvg::usvg::TreeParsing;
pub mod menus;
pub mod style;
pub mod ui;
mod updates;

pub const ICON_FONT: Font = Font::with_name("icons");

pub const LOGO: &[u8; 1495] = include_bytes!("../assets/marcopad.svg");

fn window_icon() -> Option<window::Icon> {
    #[cfg(target_os = "windows")]
    {
        let rtree = resvg::usvg::Tree::from_data(LOGO, &resvg::usvg::Options::default()).unwrap();
        let mut tree = resvg::Tree::from_usvg(&rtree);
        tree.size = resvg::usvg::Size::from_wh(64.0, 64.0).unwrap();

        let mut buffer: [u8; 64 * 64 * 4] = [0; 64 * 64 * 4];

        let mut pixmap = resvg::tiny_skia::PixmapMut::from_bytes(&mut buffer, 64, 64).unwrap();
        tree.render(resvg::usvg::Transform::default(), &mut pixmap);

        window::icon::from_rgba(buffer.to_vec(), 64, 64).ok()
    }
    #[cfg(not(target_os = "windows"))]
    None
}
fn main() -> iced::Result {
    App::run(Settings {
        default_font: Font::with_name("Omnes Pro"),
        window: window::Settings {
            size: (1280, 720),
            min_size: Some((800, 600)),
            resizable: true,
            icon: window_icon(),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),
    Settings(bool),
    ThemeChanged(bool),
    EditChanged,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Menu {
    Main,
    Settings,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Edited {
    Macros,
    Rgb,
}

pub struct App {
    menu: Menu,
    edited: Edited,
    when_edited: Instant,
    theme_light: bool,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu: Menu::Main,
            edited: Edited::Macros,
            when_edited: Instant::now(),
            theme_light: false,
            theme: Theme::Dark,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let omnes = font::load(include_bytes!("../assets/Omnes Pro Regular.otf").as_slice())
            .map(Message::FontLoaded);
        let icons =
            font::load(include_bytes!("../assets/icons.ttf").as_slice()).map(Message::FontLoaded);
        (Self::default(), Command::batch(vec![omnes, icons]))
    }

    fn title(&self) -> String {
        "MarcoPad™ MacroMaker".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        updates::update(self, message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.menu {
            Menu::Main => menus::main::view(self),
            Menu::Settings => menus::settings::view(self),
        }
    }
}
