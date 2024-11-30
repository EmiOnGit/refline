use std::time::{Duration, Instant};

use cosmic::iced::keyboard::{self, Key};
use tracing::info;

use crate::{app::Message, reference::Reference};

#[derive(Debug)]
pub struct FigureDrawingState {
    pub current_ref: Option<usize>,
    pub history: Vec<Reference>,
    pub sfw_only: bool,
    pub duration_per_image: Duration,
    pub last_fetched: Instant,
}
impl Default for FigureDrawingState {
    fn default() -> Self {
        FigureDrawingState {
            current_ref: None,
            history: Vec::new(),
            sfw_only: true,
            duration_per_image: Duration::from_secs(1 * 60),
            last_fetched: Instant::now(),
        }
    }
}
pub fn keypress(key_press: Key) -> Option<Message> {
    match key_press {
        keyboard::Key::Named(_name) => None,
        keyboard::Key::Character(c) => {
            info!("registered keyboard input: {c}");
            let c = c.chars().next().unwrap();
            if c == 'l' {
                Some(Message::IncreaseReferenceCounter { amount: 1 }.into())
            } else if c == 'h' {
                Some(Message::IncreaseReferenceCounter { amount: -1 }.into())
            } else {
                None
            }
        }
        keyboard::Key::Unidentified => {
            tracing::warn!("unidentified keyboard press");
            None
        }
    }
}
