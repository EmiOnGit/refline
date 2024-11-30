use cosmic::iced::widget::pane_grid::{self};
use cosmic::iced::widget::{button, column, row, scrollable, text};
use cosmic::iced::{keyboard, widget};
use cosmic::iced::{Center, Color, Element, Fill, Size};

use crate::app::{self, Message};
use crate::reference_board::ReferenceBoard;

struct Example {
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::{Axis, Direction};

    match key.as_ref() {
        Key::Character("v") => Some(Message::SplitFocused(Axis::Vertical)),
        Key::Character("h") => Some(Message::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(Message::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(|_| Message::FocusAdjacent)
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

pub fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> cosmic::Element<'a, app::Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = column![
        button(
            "Split horizontally",
            Message::Split(pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split vertically",
            Message::Split(pane_grid::Axis::Vertical, pane),
        )
    ]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(button("Close", Message::Close(pane)))
    } else {
        None
    })
    .spacing(5)
    .max_width(160);

    let content = column![text!("{}x{}", size.width, size.height).size(24), controls,]
        .spacing(10)
        .align_x(Center);

    cosmic::widget::container(scrollable(content))
        .center_y(Fill)
        .padding(5)
        .into()
}

pub fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, app::Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::Restore)
        } else {
            ("Maximize", Message::Maximize(pane))
        };

        Some(button(text(content).size(14)).padding(3).on_press(message))
    } else {
        None
    });

    let close = button(text("Close").size(14)).padding(3).on_press_maybe(
        if total_panes > 1 && !is_pinned {
            Some(Message::Close(pane))
        } else {
            None
        },
    );

    row.push(close).into()
}

mod style {
    use cosmic::iced::widget::container;
    use cosmic::iced::{Background, Border};
    use cosmic::Theme;

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.cosmic();

        container::Style {
            text_color: Some(palette.primary.component.selected_text.color.into()),
            background: Some(Background::Color(palette.background.base.color.into())),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.cosmic();

        container::Style {
            text_color: Some(palette.primary.component.selected_text.color.into()),
            background: Some(Background::Color(palette.background.base.color.into())),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.cosmic();

        container::Style {
            background: Some(Background::Color(
                palette.background.component.base.color.into(),
            )),
            border: Border {
                width: 2.0,
                color: palette.background.component.border.color.into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.cosmic();

        container::Style {
            background: Some(Background::Color(
                palette.background.component.base.color.into(),
            )),
            border: Border {
                width: 2.0,
                color: palette.background.component.border.color.into(),
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
