use std::path::PathBuf;

use cosmic::{
    iced::keyboard::{self, Key},
    widget::pane_grid::{self, Axis},
};

use crate::app::Message;

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
