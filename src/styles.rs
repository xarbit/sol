use cosmic::iced::{Background, Border, Color, Shadow, Vector};
use cosmic::widget::container;

/// Style for the overlay sidebar in mobile/condensed mode
pub fn overlay_sidebar_style(theme: &cosmic::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            theme.cosmic().background.base.into(),
        )),
        border: Border {
            width: 0.0,
            ..Default::default()
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: Vector::new(2.0, 0.0),
            blur_radius: 10.0,
        },
        ..Default::default()
    }
}
