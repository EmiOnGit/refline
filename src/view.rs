use crate::app::Message;
use crate::app::{self, AppModel};
use crate::fl;
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Length;
use cosmic::iced_widget::{button, row};
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
    let toggler = widget::toggler(figure_drawing_state.sfw_only).on_toggle(Message::SetSfwFilter);
    let image = widget::Image::new(handle);
    widget::column()
        .push(row![widget::text("sfw_filter_active"), toggler])
        .push(image)
        .into()
}
pub fn reference_board(app: &AppModel) -> Element<app::Message> {
    todo!()
}
pub fn reference_store(app: &AppModel) -> Element<app::Message> {
    let mut grid = widget::Grid::new();
    grid = grid.push(button(widget::text(fl!("add_source"))).on_press(Message::AddFilesToRefStore));
    grid = grid.insert_row();
    grid = grid.push(widget::text("path"));
    grid = grid.push(widget::text("is_sfw"));
    grid = grid.push(widget::text(fl!("remove_source")));
    grid = grid.insert_row();
    for source in &app.ref_store.source_folders {
        grid = grid.push(widget::text(format!("{:?}", &source.path)));
        grid = grid.push(
            widget::toggler(source.is_sfw)
                .on_toggle(|is_sfw| Message::SetSfwSource(is_sfw, source.path.clone())),
        );
        grid = grid.push(button(widget::text("x")).on_press(Message::RemoveSource(source.clone())));
        grid = grid.insert_row();
    }
    grid.into()
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
