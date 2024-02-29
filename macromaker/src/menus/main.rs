use crate::{
    actions::Action,
    selected_button,
    style::ContainerStyle,
    ui::{nav_button, settings_icon},
    App, Menu, Message, BUTTONS, CONNECTION,
};
use firmware::State;
use iced::{
    theme,
    widget::{button, text_input, Svg},
    Element, Length, Padding, Theme,
};
use iced::{
    widget::{
        button::Appearance, column, container, horizontal_space, pick_list, row, svg, text, Rule,
        Space, Text,
    },
    Background, Color,
};

type Out<'a> = Element<'a, Message, Theme>;
pub fn view(app: &App) -> Out {
    let logo_handle = svg::Handle::from_memory(crate::SVG_LOGO.as_slice());

    let logo = svg(logo_handle).width(60).height(60);

    let top_bar = top_bar(logo);
    let bottom_bar = bottom_bar();

    let middle = middle(app.recording);

    container(column![top_bar, middle, bottom_bar])
        .padding(Padding::new(7.0))
        .into()
}

fn top_bar<'a>(logo: Svg) -> Out<'a> {
    row![
        logo,
        horizontal_space(),
        text("marcoPad").size(50),
        horizontal_space(),
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

fn middle<'a>(recording: bool) -> Out<'a> {
    let connection = unsafe {
        if CONNECTION.is_some() {
            text("connected")
        } else {
            text("not connected")
        }
    };
    let selected_button = BUTTONS.lock()[selected_button() - 1].clone();
    container(row![
        column![connection, buttons(),],
        programming_panel(&selected_button, recording)
    ])
    .height(Length::Fill)
    .width(Length::Fill)
    .padding(Padding::new(20.0))
    .style(theme::Container::Custom(Box::new(ContainerStyle)))
    .into()
}

fn buttons<'a>() -> Out<'a> {
    let p = BUTTONS.lock().clone().map(|button| button.state);
    container(
        container(column![
            row!(
                pad_button(1, p[0]),
                pad_button(2, p[1]),
                pad_button(3, p[2]),
            ),
            row!(
                pad_button(4, p[3]),
                pad_button(5, p[4]),
                pad_button(6, p[5]),
            ),
            row!(
                pad_button(7, p[6]),
                pad_button(8, p[7]),
                pad_button(9, p[8]),
            ),
        ])
        .width(340)
        .height(340)
        .padding(20)
        .style(theme::Container::Custom(Box::new(ContainerStyle))),
    )
    .padding(Padding::from([30.0, 30.0, 10.0, 10.0]))
    .into()
}

struct PressedButtonStyle;
impl button::StyleSheet for PressedButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        Appearance {
            background: Some(Background::Color(Color::from_rgb8(50, 32, 212))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}
struct PhysicallyPressedButtonStyle;
impl button::StyleSheet for PhysicallyPressedButtonStyle {
    type Style = iced::Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        Appearance {
            background: Some(Background::Color(Color::from_rgb8(200, 32, 112))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }
}

fn pad_button<'a>(id: usize, state: State) -> Out<'a> {
    let pressed = id == selected_button();
    let mut button = button(Text::new(id.to_string()))
        .width(90.0)
        .height(90.0)
        .on_press(Message::Button(id));
    if state == State::Pressed {
        button = button.style(theme::Button::custom(PhysicallyPressedButtonStyle));
    } else if pressed {
        button = button.style(theme::Button::custom(PressedButtonStyle));
    }

    container(button).padding(5).into()
}

fn programming_panel<'a>(button: &crate::Button, recording: bool) -> Out<'a> {
    let id = selected_button();
    if id == 0 {
        return Space::new(0, 0).into();
    }
    let pick_list = pick_list(
        Action::ALL.to_vec(),
        Some(button.action.clone()),
        Message::EditButton,
    )
    .placeholder("action");
    container(row![
        Rule::vertical(20),
        column![
            text(format!("Editing button {id}")).size(30),
            Space::new(0, 10),
            pick_list,
            Space::new(0, 20),
            action_menu(button, recording),
        ],
    ])
    .padding(5)
    .into()
}

fn action_menu<'a>(key: &crate::Button, recording: bool) -> Out<'a> {
    match &key.action {
        Action::Nothing => container(text("This button does nothing.")),
        Action::Keypress(key) => container(column![
            text(format!("This key simulates key \"{key:?}\".")),
            {
                if recording {
                    button(
                        Text::new("Recording key...")
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center),
                    )
                    .width(220)
                    .height(90)
                } else {
                    button(
                        Text::new("Record key")
                            .horizontal_alignment(iced::alignment::Horizontal::Center)
                            .vertical_alignment(iced::alignment::Vertical::Center),
                    )
                    .on_press(Message::RecordKey)
                    .width(220)
                    .height(90)
                }
            },
        ]),
        Action::Command(command) => container(column![
            text("Executes the given terminal command on press:"),
            text_input("", command).on_input(Message::EditCommand)
        ]),
        _ => container(text("")),
    }
    .into()
}
