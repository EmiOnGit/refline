use std::time::{Duration, Instant};

use crate::reference::Reference;

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
            sfw_only: false,
            duration_per_image: Duration::from_secs(1 * 60),
            last_fetched: Instant::now(),
        }
    }
}
