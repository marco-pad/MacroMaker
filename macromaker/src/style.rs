use iced::widget::container;
use iced::{Border, Color, Theme};

pub struct ContainerStyle;

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let border = Border {
            color: Color::from_rgb(0.4, 0.4, 0.4),
            width: 3.0,
            radius: 4.0.into(),
        };
        container::Appearance {
            border,
            text_color: Default::default(),
            background: Default::default(),
            shadow: Default::default(),
        }
    }
}
