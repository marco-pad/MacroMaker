use iced::{Element, Padding, theme};
use iced::widget::{tooltip, container, text, button};
use crate::{Message, Menu, ICON_FONT};

fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    text(codepoint).font(ICON_FONT).size(30).into()
}

pub fn settings_icon<'a>() -> Element<'a, Message> {
    icon('\u{E800}')
}

pub fn back_icon<'a>() -> Element<'a, Message> {
    icon('\u{E801}')
}

pub fn nav_button<'a>(menu: Menu, tip: &str, icon: Element<'a, Message>) -> Element<'a, Message> {
    tooltip(
        button(container(icon).width(40.0).center_x())
            .on_press(Message::Settings(menu.eq(&Menu::Settings)))
            .padding(Padding::new(10.0)),
        tip,
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}