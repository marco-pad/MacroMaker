use crate::{
    ui::{back_icon, nav_button},
    App, Menu, Message,
};
use iced::widget::{checkbox, column, container, horizontal_space, row, text};
use iced::{Element, Length, Padding, Renderer, Theme};

pub fn view<'a>(app: &App) -> Element<'a, Message, Renderer<Theme>> {
    container(column![
        row![
            container(text("Settings").size(50)).padding(Padding::from([5.0, 0.0, 10.0, 0.0])),
            horizontal_space(Length::Fill),
            nav_button(Menu::Main, "Back", back_icon())
        ],
        checkbox("Light Mode", app.theme_light, Message::ThemeChanged),
    ])
    .padding(Padding::new(7.0))
    .into()
}
