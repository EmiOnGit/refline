use crate::app::Message;
use crate::app::{self, AppModel};
use crate::fl;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::iced_widget::button;
use cosmic::widget::{self};
use cosmic::{Apply, Element};

pub fn figure_drawing(app: &AppModel) -> Element<app::Message> {
    let ref_store = &app.ref_store;
    let figure_drawing_state = &app.figure_drawing_state;
    let Some(index) = figure_drawing_state.current_ref else {
        tracing::warn!("no current_ref_pointer not set");
        return center_text(fl!("add_refs"));
    };
    let Some(reference) = &figure_drawing_state.history.get(index) else {
        tracing::error!("index points to invalid history point");
        return center_text(fl!("add_refs"));
    };
    let Some(img) = ref_store.ref_data.get(&reference.path).cloned() else {
        tracing::warn!("image not loaded yet");
        return center_text(fl!("loading"));
    };
    let handle =
        cosmic::widget::image::Handle::from_rgba(img.width(), img.height(), img.into_vec());
    widget::Image::new(handle).into()
}
pub fn reference_board(app: &AppModel) -> Element<app::Message> {
    todo!()
}
pub fn reference_store(app: &AppModel) -> Element<app::Message> {
    button("press").on_press(Message::AddFilesToRefStore).into()
}
pub fn center_text(text: String) -> Element<'static, <AppModel as cosmic::Application>::Message> {
    widget::text::title1(text)
        .apply(widget::container)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}
