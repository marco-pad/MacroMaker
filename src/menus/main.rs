use crate::{
    style::ContainerStyle,
    ui::{nav_button, settings_icon},
    App, Edited, Menu, Message,
};
use iced::widget::{column, container, horizontal_space, row, svg, text, vertical_space};
use iced::{
    theme,
    widget::{button, tooltip, Svg},
    Element, Length, Padding, Renderer, Theme,
};

type Out<'a> = Element<'a, Message, Renderer<Theme>>;
pub fn view(app: &App) -> Out {
    let logo_handle = svg::Handle::from_memory(crate::LOGO.as_slice());

    let logo = svg(logo_handle).width(60).height(60);

    let top_bar = top_bar(logo);
    let bottom_bar = bottom_bar();

    let middle = middle(app.edited);

    container(column![top_bar, middle, bottom_bar])
        .padding(Padding::new(7.0))
        .into()
}

fn top_bar<'a>(logo: Svg) -> Out<'a> {
    row![
        logo,
        horizontal_space(Length::Fill),
        text("marcoPad").size(50),
        horizontal_space(Length::Fill),
        nav_button(Menu::Settings, "Settings", settings_icon())
    ]
    .into()
}

fn bottom_bar<'a>() -> Out<'a> {
    row![container(text("By Tim, Jakob and Jonathan"))
        .width(Length::Fill)
        .center_x()]
    .into()
}

fn middle<'a>(edited: Edited) -> Out<'a> {
    container(row![
        column![toggle_button(edited), buttons(),],
        programming_panel()
    ])
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(Padding::new(20.0))
    .style(theme::Container::Custom(Box::new(ContainerStyle)))
    .into()
}

fn toggle_button<'a>(edited: Edited) -> Out<'a> {
    let text = match edited {
        Edited::Macros => "Edit RGB lighting",
        Edited::Rgb => "Edit macros and commands",
    };
    tooltip(
        button(text).padding(10).on_press(Message::EditChanged),
        text,
        tooltip::Position::FollowCursor,
    )
    .style(theme::Container::Box)
    .into()
}

fn buttons<'a>() -> Out<'a> {
    container(
        container(row![
            column!(pad_button(), pad_button(), pad_button(),),
            column!(pad_button(), pad_button(), pad_button(),),
            column!(pad_button(), pad_button(), pad_button(),),
        ])
        .width(340)
        .height(340)
        .padding(20)
        .style(theme::Container::Custom(Box::new(ContainerStyle))),
    )
    .padding(Padding::from([30.0, 30.0, 10.0, 10.0]))
    .into()
}

fn pad_button<'a>() -> Out<'a> {
    container(button("bye bye welt").width(90.0).height(90.0))
        .padding(5)
        .into()
}

fn programming_panel<'a>() -> Out<'a> {
    container(column![text("Hallo! Programming panel!")])
        .padding(5)
        .into()
}
