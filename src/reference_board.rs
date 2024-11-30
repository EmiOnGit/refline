use std::path::{Path, PathBuf};

use cosmic::{
    iced::{
        keyboard::{self, Key},
        widget,
        Length::Fill,
        Size,
    },
    iced_widget::{button, row},
    widget::{
        pane_grid::{self, Axis},
        scrollable, text,
    },
};

use crate::app::{self, Message};

pub const REF_BOARD_FILENAME: &str = "refboard.ron";
pub struct ReferenceBoard {
    pub panes: pane_grid::State<ReferenceNode>,
    pub focus: Option<pane_grid::Pane>,
    pub panes_created: usize,
}
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ReferenceNode {
    pub id: usize,
    pub is_pinned: bool,
    pub path: PathBuf,
}

impl ReferenceBoard {
    // pub fn try_load() -> Option<ReferenceBoard> {
    //     io::try_load(REF_BOARD_FILENAME)
    // }
    // pub fn save_to_disk(&self) -> Option<()> {
    //     io::save_to_disk(self, REF_BOARD_FILENAME)
    // }
}
impl Default for ReferenceBoard {
    fn default() -> Self {
        let (pane_state, _) = pane_grid::State::new(ReferenceNode {
            path: "/home/emi/refs/dancing_blonde/DSC_6801.jpg".into(),
            id: 0,
            is_pinned: false,
        });
        ReferenceBoard {
            panes: pane_state,
            focus: None,
            panes_created: 1,
        }
    }
}
pub fn keypress(key_press: Key) -> Option<Message> {
    match key_press.as_ref() {
        keyboard::Key::Character("v") => Some(Message::SplitFocused(Axis::Vertical)),
        keyboard::Key::Character("h") => Some(Message::SplitFocused(Axis::Horizontal)),
        keyboard::Key::Character("w") => Some(Message::CloseFocused),
        keyboard::Key::Named(key) => {
            let direction = match key {
                keyboard::key::Named::ArrowUp => Some(pane_grid::Direction::Up),
                keyboard::key::Named::ArrowDown => Some(pane_grid::Direction::Down),
                keyboard::key::Named::ArrowLeft => Some(pane_grid::Direction::Left),
                keyboard::key::Named::ArrowRight => Some(pane_grid::Direction::Right),
                _ => None,
            };

            direction.map(|_| Message::FocusAdjacent)
        }
        _ => None,
    }
}
pub fn view_content<'a>(image_path: &Path) -> cosmic::Element<'a, app::Message> {
    let content = widget::image(image_path);

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
) -> cosmic::Element<'a, app::Message> {
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
