use iced::{Theme, Color};
use iced::widget::container;


pub struct ContainerStyle;

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 4.0.into(),
            border_width: 3.0,
            border_color: Color::from_rgb(0.4, 0.4, 0.4),
            ..Default::default()
        }
    }
}
