#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::Cursor, sync::atomic::AtomicUsize};

use button::Button;
use connection::Connection;
use iced::{executor, font, window, Font};
use iced::{Application, Command, Renderer, Settings, Theme};
use parking_lot::Mutex;

use updates::Message;

// Modules
mod actions;
mod button;
mod connection;
mod menus;
mod style;
mod ui;
mod updates;

pub const ICON_FONT: Font = Font::with_name("icons");

pub const LOGO: &[u8; 4981] = include_bytes!("../assets/marcopad.png");
pub const SVG_LOGO: &[u8; 1495] = include_bytes!("../assets/marcopad.svg");

static SELECTED_BUTTON: AtomicUsize = AtomicUsize::new(1);
pub fn select_button(id: usize) {
    SELECTED_BUTTON.store(id, std::sync::atomic::Ordering::Release);
}
pub fn selected_button() -> usize {
    SELECTED_BUTTON.load(std::sync::atomic::Ordering::Acquire)
}

static mut CONNECTION: Option<Connection> = None;
static BUTTONS: Buttons = Mutex::new([Button::NOTHING; 9]);

type Buttons = Mutex<[Button; 9]>;

fn window_icon() -> Option<window::Icon> {
    let mut image = image::io::Reader::new(Cursor::new(LOGO));
    image.set_format(image::ImageFormat::Png);
    let image = image.decode().unwrap().into_rgba8();

    window::icon::from_rgba(image.into_vec(), 64, 64).ok()
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Menu {
    Main,
    Settings,
}

pub struct App {
    menu: Menu,
    theme_light: bool,
    theme: Theme,

    recording: bool,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn new(_buttons: Self::Flags) -> (Self, Command<Self::Message>) {
        let omnes = font::load(include_bytes!("../assets/Omnes Pro Regular.otf").as_slice())
            .map(Message::FontLoaded);
        let icons =
            font::load(include_bytes!("../assets/icons.ttf").as_slice()).map(Message::FontLoaded);
        (
            Self {
                menu: Menu::Main,
                theme_light: false,
                theme: Theme::Dark,
                recording: false,
            },
            Command::batch(vec![
                omnes,
                icons,
                Command::perform(async {}, |_| Message::Input),
            ]),
        )
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
    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let regular_update =
            iced::time::every(std::time::Duration::from_millis(100)).map(|_| Message::Nothing);
        let input = iced::subscription::events().map(Message::Event);
        iced::Subscription::batch(vec![regular_update, input])
    }
}
