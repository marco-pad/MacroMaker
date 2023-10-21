#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::text;
use iced::{Application, Theme, Command, Renderer, Settings};
use iced::executor;

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Clone, Copy, Debug)]
enum Message {}

#[derive(Default)]
struct App;

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        "MarcoPad™ MacroMaker".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {

        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        text("hi").into()
    }
}