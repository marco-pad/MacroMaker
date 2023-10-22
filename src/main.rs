#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, svg, text, tooltip,
};
use iced::{executor, font, theme, window, Color, Element, Font, Padding};
use iced::{Application, Command, Length, Renderer, Settings, Theme};
use resvg::usvg::TreeParsing;

const ICON_FONT: Font = Font::with_name("icons");

const LOGO: &[u8; 1495] = include_bytes!("../assets/marcopad.svg");

fn window_icon() -> Option<window::Icon> {
    let rtree = resvg::usvg::Tree::from_data(LOGO, &resvg::usvg::Options::default()).unwrap();
    let mut tree = resvg::Tree::from_usvg(&rtree);
    tree.size = resvg::usvg::Size::from_wh(64.0, 64.0).unwrap();

    let mut buffer: [u8; 64 * 64 * 4] = [0; 64 * 64 * 4];

    let mut pixmap = resvg::tiny_skia::PixmapMut::from_bytes(&mut buffer, 64, 64).unwrap();
    tree.render(resvg::usvg::Transform::default(), &mut pixmap);

    window::icon::from_rgba(buffer.to_vec(), 64, 64).ok()
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
enum Message {
    FontLoaded(Result<(), font::Error>),
    Settings(bool),
    ThemeChanged(bool),
}

#[derive(Clone, Copy, Debug)]
enum Menu {
    Main,
    Settings,
}

struct App {
    menu: Menu,
    theme_light: bool,
    theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            menu: Menu::Main,
            theme_light: false,
            theme: Theme::Dark,
        }
    }
}

struct ContainerStyle;

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 10.0.into(),
            border_width: 3.0,
            border_color: Color::from_rgb(0.4, 0.4, 0.4),
            ..Default::default()
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
        match message {
            Message::FontLoaded(_) => Command::none(),
            Message::Settings(val) => {
                self.menu = val.then_some(Menu::Settings).unwrap_or(Menu::Main);
                Command::none()
            }
            Message::ThemeChanged(val) => {
                self.theme_light = val;
                self.theme = self
                    .theme_light
                    .then_some(Theme::Light)
                    .unwrap_or(Theme::Dark);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Renderer<Self::Theme>> {
        match self.menu {
            Menu::Main => {
                let logo_handle = svg::Handle::from_memory(LOGO.as_slice());

                let logo = svg(logo_handle).width(60).height(60);

                let top_bar = row![
                    logo,
                    horizontal_space(Length::Fill),
                    text("marcoPad").size(50),
                    horizontal_space(Length::Fill),
                    settings_button()
                ];

                let middle = container(text("middle"))
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .style(iced::theme::Container::Custom(Box::new(ContainerStyle)));

                let bottom_bar = row![container(text("By Tim, Jakob and Jonathan"))
                    .width(Length::Fill)
                    .center_x()];

                container(column![top_bar, middle, bottom_bar])
                    .padding(Padding::new(5.0))
                    .into()
            }
            Menu::Settings => container(column![
                row![
                    container(text("Settings").size(50)).padding(Padding::from([5.0, 0.0, 10.0, 0.0])),
                    horizontal_space(Length::Fill),
                    back_button()
                ],
                checkbox("Light Mode", self.theme_light, |val| Message::ThemeChanged(
                    val
                )),
            ])
            .padding(Padding::new(10.0))
            .into(),
        }
    }
}

fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    text(codepoint).font(ICON_FONT).size(30).into()
}

fn settings_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

fn back_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

fn settings_button<'a>() -> Element<'a, Message> {
    tooltip(
        button(container(settings_icon()).width(40.0).center_x())
            .on_press(Message::Settings(true))
            .padding(Padding::new(10.0)),
        "Settings",
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}

fn back_button<'a>() -> Element<'a, Message> {
    tooltip(
        button(container(back_icon()).width(40.0).center_x())
            .on_press(Message::Settings(false))
            .padding(Padding::new(10.0)),
        "back",
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}
