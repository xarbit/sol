use cosmic::iced::{Background, Border, Color, Shadow, Vector};
use cosmic::widget::container;
use crate::ui_constants::{SHADOW_OPACITY, SHADOW_OFFSET_X, SHADOW_OFFSET_Y, SHADOW_BLUR_RADIUS};

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
            color: Color::from_rgba(0.0, 0.0, 0.0, SHADOW_OPACITY),
            offset: Vector::new(SHADOW_OFFSET_X, SHADOW_OFFSET_Y),
            blur_radius: SHADOW_BLUR_RADIUS,
        },
        ..Default::default()
    }
}
