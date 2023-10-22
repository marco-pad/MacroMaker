use crate::{Message, App, Menu, ui::{nav_button, back_icon}};
use iced::{Padding, Length, Element, Renderer, Theme};
use iced::widget::{
    checkbox, column, container, horizontal_space, row, text
};

pub fn view<'a>(app: &App) -> Element<'a, Message, Renderer<Theme>> {
    container(column![
        row![
            container(text("Settings").size(50)).padding(Padding::from([5.0, 0.0, 10.0, 0.0])),
            horizontal_space(Length::Fill),
            nav_button(Menu::Main, "Back", back_icon())
        ],
        checkbox("Light Mode", app.theme_light, |val| Message::ThemeChanged(
            val
        )),
    ])
    .padding(Padding::new(7.0))
    .into()
}