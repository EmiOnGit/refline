use crate::app::Message;
use crate::app::{self, AppModel};
use crate::fl;
use crate::reference_pane::{view_content, view_controls};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::Color;
use cosmic::iced::Length::{self, Fill};
use cosmic::iced_widget::{button, row};
use cosmic::widget::{self, pane_grid, responsive, text, PaneGrid};
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
const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);
pub fn reference_board(app: &AppModel) -> Element<app::Message> {
    let board = &app.reference_board;

    let focus = board.focus;
    let total_panes = board.panes_created;

    let pane_grid = PaneGrid::new(&board.panes, |id, pane, is_maximized| {
        let is_focused = focus == Some(id);

        let pin_button = button(text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14))
            .on_press(Message::TogglePin(id))
            .padding(3);

        let title = row![
            pin_button,
            "Pane",
            text(pane.id.to_string()),
            //     if is_focused {
            //     PANE_ID_COLOR_FOCUSED.into()
            // } else {
            //     PANE_ID_COLOR_UNFOCUSED.into()
            // }),
        ]
        .spacing(5);

        let title_bar_controls = widget::pane_grid::Controls::dynamic(
            view_controls(id, total_panes, pane.is_pinned, is_maximized),
            button(text("X").size(14)).padding(3).on_press_maybe(
                if total_panes > 1 && !pane.is_pinned {
                    Some(Message::Close(id))
                } else {
                    None
                },
            ),
        );
        let title_bar = widget::pane_grid::TitleBar::new(title)
            .controls(title_bar_controls)
            .padding(10);

        widget::pane_grid::Content::new(responsive(move |size| {
            view_content(id, total_panes, pane.is_pinned, size)
        }))
        .title_bar(title_bar)
    })
    .width(Fill)
    .height(Fill)
    .spacing(10)
    .on_click(|_| Message::Clicked)
    .on_drag(|_| Message::Dragged)
    .on_resize(10, |_| Message::Resized);

    let cont = widget::container(pane_grid)
        .width(Fill)
        .height(Fill)
        .padding(10);

    cont.into()
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
