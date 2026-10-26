use iced::{
    Background, Theme, color,
    widget::{button, container},
};

pub fn top_level_container_style(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color!(0x1A1A1A))),
        ..Default::default()
    }
}

pub fn panel_container_style(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(color!(0x2D2B2B))),
        border: iced::Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn side_panel_button_style(_: &Theme, status: button::Status) -> button::Style {
    let background = match status {
        button::Status::Active => Background::Color(color!(0x4B4B4B)),
        button::Status::Hovered => Background::Color(color!(0xBE9F48)),
        button::Status::Pressed => Background::Color(color!(0x9C8135)),
        button::Status::Disabled => Background::Color(color!(0xBE9F48)),
    };

    button::Style {
        background: Some(background),
        border: iced::Border {
            radius: 5.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
