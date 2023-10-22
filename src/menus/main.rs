use crate::{App, style, ui::{nav_button, settings_icon}, Menu, Message, Edited};
use iced::{Element, Padding, Renderer, Length, theme, widget::{Svg, tooltip, button}, Theme};
use iced::widget::{row, text, column, svg, container, horizontal_space};

pub fn view<'a>(app: &App) -> Element<'a, Message, Renderer<Theme>> {
    let logo_handle = svg::Handle::from_memory(crate::LOGO.as_slice());

    let logo = svg(logo_handle).width(60).height(60);

    let top_bar = top_bar(logo);
    let bottom_bar = bottom_bar();

    let middle = middle(app.edited);


    container(column![top_bar, middle, bottom_bar])
        .padding(Padding::new(7.0))
        .into()
}

fn top_bar<'a>(logo: Svg) -> Element<'a, Message, Renderer<Theme>> {
    row![
        logo,
        horizontal_space(Length::Fill),
        text("marcoPad").size(50),
        horizontal_space(Length::Fill),
        nav_button(Menu::Settings, "Settings", settings_icon())
    ].into()
}

fn bottom_bar<'a>() -> Element<'a, Message, Renderer<Theme>> {
    row![
        container(text("By Tim, Jakob and Jonathan"))
        .width(Length::Fill)
        .center_x()
    ].into()
}

fn middle<'a>(edited: Edited) -> Element<'a, Message, Renderer<Theme>> {
    container(toggle_button(edited))
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(Padding::new(8.0))
        .style(theme::Container::Custom(Box::new(style::ContainerStyle)))
        .into()
}

fn toggle_button<'a>(edited: Edited) -> Element<'a, Message, Renderer<Theme>> {
    let text = match edited {
        Edited::Macros => "Edit RGB lighting",
        Edited::Rgb => "Edit macros and commands",
    };
    tooltip(
        button(
            text
        )
        .padding(10)
        .on_press(Message::EditChanged),
        text,
        tooltip::Position::FollowCursor
    )
    .into()
}