use crate::{
    style::ContainerStyle,
    ui::{nav_button, settings_icon},
    App, Edited, Menu, Message,
};
use iced::widget::{column, container, horizontal_space, row, svg, text, Rule, Space, Text};
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

    let middle = middle(app.edited, app.selected_button);

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

fn middle<'a>(edited: Edited, selected: usize) -> Out<'a> {
    container(row![
        column![toggle_button(edited), buttons(),],
        programming_panel(selected)
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
        container(column![
            row!(pad_button(1), pad_button(2), pad_button(3),),
            row!(pad_button(4), pad_button(5), pad_button(6),),
            row!(pad_button(7), pad_button(8), pad_button(9),),
        ])
        .width(340)
        .height(340)
        .padding(20)
        .style(theme::Container::Custom(Box::new(ContainerStyle))),
    )
    .padding(Padding::from([30.0, 30.0, 10.0, 10.0]))
    .into()
}

fn pad_button<'a>(id: usize) -> Out<'a> {
    container(
        button(Text::new(id.to_string()))
            .width(90.0)
            .height(90.0)
            .on_press(Message::Button(id)),
    )
    .padding(5)
    .into()
}

fn programming_panel<'a>(button_id: usize) -> Out<'a> {
    if button_id == 0 {
        return Space::new(0, 0).into();
    }
    container(row![
        Rule::vertical(20),
        column![text(format!("Editing button {button_id}."))],
    ])
    .padding(5)
    .into()
}
