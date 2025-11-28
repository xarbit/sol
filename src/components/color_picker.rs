use cosmic::iced::{Border, Color, Length};
use cosmic::widget::{button, column, container, grid, row};
use cosmic::{widget, Element};

use crate::message::Message;

/// Size of the color indicator button
pub const COLOR_INDICATOR_SIZE: f32 = 24.0;

/// Predefined color palette for calendars
pub const CALENDAR_COLORS: &[(&str, &str)] = &[
    // Blues
    ("#3B82F6", "Blue"),
    ("#0EA5E9", "Sky Blue"),
    ("#2563EB", "Royal Blue"),
    ("#1E40AF", "Dark Blue"),
    // Purples
    ("#8B5CF6", "Purple"),
    ("#A78BFA", "Light Purple"),
    ("#7C3AED", "Violet"),
    ("#6D28D9", "Deep Purple"),
    // Greens
    ("#10B981", "Green"),
    ("#34D399", "Emerald"),
    ("#059669", "Dark Green"),
    ("#047857", "Forest Green"),
    // Yellows/Oranges
    ("#F59E0B", "Amber"),
    ("#FBBF24", "Yellow"),
    ("#F97316", "Orange"),
    ("#EA580C", "Dark Orange"),
    // Reds/Pinks
    ("#EF4444", "Red"),
    ("#F87171", "Light Red"),
    ("#DC2626", "Crimson"),
    ("#EC4899", "Pink"),
    // Others
    ("#6366F1", "Indigo"),
    ("#14B8A6", "Teal"),
    ("#06B6D4", "Cyan"),
    ("#6B7280", "Gray"),
];

/// Render a color indicator button
pub fn render_color_indicator<'a>(
    calendar_id: String,
    current_color: &str,
    size: f32,
) -> Element<'a, Message> {
    let color = parse_hex_color(current_color).unwrap_or(Color::from_rgb8(107, 114, 128));

    button::custom(
        container(widget::text(""))
            .width(size)
            .height(size)
            .style(move |_theme: &cosmic::Theme| {
                cosmic::iced::widget::container::Style {
                    background: Some(cosmic::iced::Background::Color(color)),
                    border: Border {
                        radius: (size / 2.0).into(),
                        width: 2.0,
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                    },
                    ..Default::default()
                }
            })
    )
    .on_press(Message::OpenColorPicker(calendar_id))
    .padding(0)
    .into()
}

/// Render a color picker palette with all available colors
pub fn render_color_palette<'a>(calendar_id: String) -> Element<'a, Message> {
    let mut color_grid = column().spacing(8);

    // Split colors into rows of 6
    for row_colors in CALENDAR_COLORS.chunks(6) {
        let mut color_row = row().spacing(8);

        for (hex, name) in row_colors {
            let color = parse_hex_color(hex).unwrap_or(Color::from_rgb8(107, 114, 128));
            let hex_owned = hex.to_string();
            let calendar_id_clone = calendar_id.clone();

            let color_button = button::custom(
                container(widget::text(""))
                    .width(32.0)
                    .height(32.0)
                    .style(move |_theme: &cosmic::Theme| {
                        cosmic::iced::widget::container::Style {
                            background: Some(cosmic::iced::Background::Color(color)),
                            border: Border {
                                radius: 16.0.into(),
                                width: 2.0,
                                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                            },
                            ..Default::default()
                        }
                    })
            )
            .on_press(Message::ChangeCalendarColor(calendar_id_clone, hex_owned))
            .padding(0);

            color_row = color_row.push(
                container(color_button)
                    .width(Length::Fixed(36.0))
                    .height(Length::Fixed(36.0))
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
            );
        }

        color_grid = color_grid.push(color_row);
    }

    container(color_grid)
        .padding(16)
        .into()
}

/// Render a compact color picker with 20 colors in a 4x5 grid
pub fn render_quick_color_picker<'a>(
    calendar_id: String,
    current_color: &str,
) -> Element<'a, Message> {
    // 20 colors organized by category - 5 colors per row, 4 rows
    let color_rows = [
        // Row 1: Blues
        ["#3B82F6", "#0EA5E9", "#2563EB", "#1E40AF", "#06B6D4"],
        // Row 2: Purples and Pinks
        ["#8B5CF6", "#A78BFA", "#7C3AED", "#EC4899", "#DB2777"],
        // Row 3: Greens and Teals
        ["#10B981", "#34D399", "#059669", "#14B8A6", "#0D9488"],
        // Row 4: Oranges, Yellows, Reds
        ["#F59E0B", "#FBBF24", "#F97316", "#EF4444", "#DC2626"],
    ];

    let mut color_grid = column().spacing(6);

    for row_colors in color_rows {
        let mut color_row = row().spacing(6);

        for hex in row_colors {
            let color = parse_hex_color(hex).unwrap_or(Color::from_rgb8(107, 114, 128));
            let hex_owned = hex.to_string();
            let calendar_id_clone = calendar_id.clone();
            let is_selected = current_color == hex;

            let border_width = if is_selected { 3.0 } else { 2.0 };
            let border_color = if is_selected {
                Color::from_rgb8(0, 0, 0)
            } else {
                Color::from_rgba(0.0, 0.0, 0.0, 0.2)
            };

            let color_button = button::custom(
                container(widget::text(""))
                    .width(28.0)
                    .height(28.0)
                    .style(move |_theme: &cosmic::Theme| {
                        cosmic::iced::widget::container::Style {
                            background: Some(cosmic::iced::Background::Color(color)),
                            border: Border {
                                radius: 14.0.into(),
                                width: border_width,
                                color: border_color,
                            },
                            ..Default::default()
                        }
                    })
            )
            .on_press(Message::ChangeCalendarColor(calendar_id_clone, hex_owned))
            .padding(0);

            color_row = color_row.push(color_button);
        }

        color_grid = color_grid.push(color_row);
    }

    color_grid.into()
}

/// Parse hex color string (e.g., "#RRGGBB") to iced Color
pub fn parse_hex_color(hex: &str) -> Result<Color, ()> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;

    Ok(Color::from_rgb8(r, g, b))
}
