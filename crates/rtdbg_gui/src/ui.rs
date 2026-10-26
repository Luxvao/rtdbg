use color_eyre::eyre::Result;
use iced::{
    Element, Length, Pixels,
    widget::{
        Button, button, column as col, container, row, rule::horizontal, svg, text_input::Icon,
    },
};

use crate::style::{panel_container_style, side_panel_button_style, top_level_container_style};

#[derive(Default)]
struct State {
    side_panel_expanded: bool,
    current_page: Page,
}

#[derive(Default)]
enum Page {
    #[default]
    Overview,
    ScriptManager,
}

#[derive(Clone)]
enum Message {
    ToggleSidePanel,
    NavigateOverview,
    NavigateScriptManager,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ToggleSidePanel => state.side_panel_expanded = !state.side_panel_expanded,
        Message::NavigateOverview => state.current_page = Page::Overview,
        Message::NavigateScriptManager => state.current_page = Page::ScriptManager,
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let side_panel_expand_button = nav_button(state, "Menu", Some(Message::ToggleSidePanel));

    let side_panel_overview_button = nav_button(
        state,
        "Overview",
        if let Page::Overview = state.current_page {
            None
        } else {
            Some(Message::NavigateOverview)
        },
    );

    let side_panel_script_manager_button = nav_button(
        state,
        "Script Manager",
        if let Page::ScriptManager = state.current_page {
            None
        } else {
            Some(Message::NavigateScriptManager)
        },
    );

    let side_panel_width = if state.side_panel_expanded {
        Length::Fixed(200.0)
    } else {
        Length::Fixed(50.0)
    };

    let side_panel = container(
        col![
            side_panel_expand_button,
            horizontal(1),
            side_panel_overview_button,
            side_panel_script_manager_button
        ]
        .spacing(Pixels::from(5))
        .padding(Pixels::from(5))
        .height(Length::Fill)
        .width(side_panel_width),
    )
    .style(panel_container_style);

    let main_panel =
        container(col![].height(Length::Fill).width(Length::Fill)).style(panel_container_style);

    container(
        row![side_panel, main_panel]
            .padding(Pixels::from(10))
            .spacing(Pixels::from(10)),
    )
    .style(top_level_container_style)
    .into()
}

pub fn init() -> Result<()> {
    iced::run(update, view)?;

    Ok(())
}

fn nav_button<'a>(state: &State, text: &'a str, on_press: Option<Message>) -> Button<'a, Message> {
    let side_panel_button_width = if state.side_panel_expanded {
        Length::Fill
    } else {
        Length::Fixed(40.0)
    };

    button(if state.side_panel_expanded { text } else { "" })
        .on_press_maybe(on_press)
        .width(side_panel_button_width)
        .height(Length::Fixed(40.0))
        .style(side_panel_button_style)
}
