// SPDX-License-Identifier: {{LICENSE}}

use crate::config::Config;
use crate::figure_drawing::FigureDrawingState;
use crate::reference::{RefStore, SourceFolder};
use crate::{fl, view};
use cosmic::app::{Core, Task};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{event, keyboard, Alignment, Subscription};
use cosmic::iced_core::Event;
use cosmic::widget::{self, icon, menu, nav_bar};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element};
use futures_util::SinkExt;
use image::RgbaImage;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

const REPOSITORY: &str = "https://github.com/pop-os/cosmic-app-template";
const APP_ICON: &[u8] = include_bytes!("../res/icons/hicolor/scalable/apps/icon.svg");

/// The application model stores app-specific state used to describe its interface and
/// drive its logic.
pub struct AppModel {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Contains items assigned to the nav bar panel.
    nav: nav_bar::Model,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    // Configuration data that persists between application runs.
    config: Config,
    /// Image references
    pub ref_store: RefStore,
    pub figure_drawing_state: FigureDrawingState,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
    OpenRepositoryUrl,
    SubscriptionChannel,
    ToggleContextPage(ContextPage),
    UpdateConfig(Config),
    AddFilesToRefStore,
    LoadNewReference,
    IncreaseReferenceCounter {
        amount: isize,
    },
    LoadedNewReference(PathBuf, RgbaImage),
    RemoveSource(SourceFolder),
    /// Can be assumed to always be of variant Message::Keypress`
    Keypress(keyboard::Event),
    SetSfwFilter(bool),
    SetSfwSource(bool, PathBuf),
}

/// Create a COSMIC application from the app model
impl Application for AppModel {
    /// The async executor that will be used to run your application's commands.
    type Executor = cosmic::executor::Default;

    /// Data that your application receives to its init method.
    type Flags = ();

    /// Messages which the application and its widgets will emit.
    type Message = Message;

    /// Unique identifier in RDNN (reverse domain name notation) format.
    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Initializes the application with any given flags and startup commands.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        // Create a nav bar with three page items.
        let mut nav = nav_bar::Model::default();

        nav.insert()
            .text(fl!("figure_drawing"))
            .data::<Page>(Page::FigureDrawing)
            .icon(icon::from_name("applications-science-symbolic"));

        nav.insert()
            .text(fl!("reference_board"))
            .data::<Page>(Page::ReferenceBoard)
            .icon(icon::from_name("applications-system-symbolic"));

        nav.insert()
            .text(fl!("reference_store"))
            .data::<Page>(Page::ReferenceStore)
            .icon(icon::from_name("applications-games-symbolic"))
            .activate();

        let mut ref_store = RefStore::try_load().unwrap_or_default();
        ref_store.sync_with_source_folders();
        // Construct the app model with the runtime's core.
        let mut app = AppModel {
            core,
            context_page: ContextPage::default(),
            nav,
            key_binds: HashMap::new(),
            // Optional configuration file for an application.
            config: cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
                .map(|context| match Config::get_entry(&context) {
                    Ok(config) => config,
                    Err((errors, config)) => {
                        for why in errors {
                            tracing::error!(%why, "error loading app config");
                        }

                        config
                    }
                })
                .unwrap_or_default(),
            ref_store,
            figure_drawing_state: FigureDrawingState::default(),
        };

        // Create a startup command that sets the window title.
        let command = app.update_title();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// Enables the COSMIC application to create a nav bar with this model.
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav)
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }

    /// Describes the interface based on the current state of the application model.
    ///
    /// Application events will be processed through the view. Any messages emitted by
    /// events received by widgets will be passed to the update method.
    fn view(&self) -> Element<Self::Message> {
        let Some(active_page): Option<&Page> = self.nav.active_data() else {
            return view::center_text(fl!("welcome"));
        };
        println!("update view{active_page:?}");
        match active_page {
            Page::FigureDrawing => view::figure_drawing(self),
            Page::ReferenceBoard => view::reference_board(self),
            Page::ReferenceStore => view::reference_store(self),
        }
    }

    /// Register subscriptions for this application.
    ///
    /// Subscriptions are long-running async tasks running in the background which
    /// emit messages to the application through a channel. They are started at the
    /// beginning of the application, and persist through its lifetime.
    fn subscription(&self) -> Subscription<Self::Message> {
        struct MySubscription;
        // .map(|f| Message::NewEvent(f));
        Subscription::batch(vec![
            event::listen_with(|ev, _status, _id| {
                let Event::Keyboard(ev) = ev else {
                    return None;
                };
                if matches!(ev, keyboard::Event::KeyPressed { .. }) {
                    return Some(Message::Keypress(ev));
                }
                None
            }),
            // Create a subscription which emits updates through a channel.
            Subscription::run_with_id(
                std::any::TypeId::of::<MySubscription>(),
                cosmic::iced::stream::channel(4, move |mut channel| async move {
                    _ = channel.send(Message::SubscriptionChannel).await;

                    futures_util::future::pending().await
                }),
            ),
            // Watch for application configuration changes.
            self.core()
                .watch_config::<Config>(Self::APP_ID)
                .map(|update| {
                    // for why in update.errors {
                    //     tracing::error!(?why, "app config error");
                    // }

                    Message::UpdateConfig(update.config)
                }),
        ])
    }

    /// Handles messages emitted by the application and its widgets.
    ///
    /// Tasks may be returned for asynchronous execution of code in the background
    /// on the application's async runtime.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        // info!("update with {message:?}");
        match message {
            Message::OpenRepositoryUrl => {
                _ = open::that_detached(REPOSITORY);
            }

            Message::SubscriptionChannel => {
                // For example purposes only.
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }

            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::AddFilesToRefStore => {
                let folders = rfd::FileDialog::new().pick_folders();
                if let Some(files) = folders {
                    self.ref_store.push_folders(&files, true)
                }
            }
            Message::LoadNewReference => {
                info!("LOAD NEW REF RECEIVED");
                let sfw_filter = self.figure_drawing_state.sfw_only;
                let count = self.ref_store.reference_count(sfw_filter);
                if count == 0 {
                    tracing::error!("Can not load new reference as no folders were added");
                    return Task::none();
                }
                let index = fastrand::usize(..count);
                let reference = self
                    .ref_store
                    .get_reference(index, sfw_filter)
                    .cloned()
                    .expect("reference count was calculated wrong");
                self.figure_drawing_state.history.push(reference.clone());
                if !self.figure_drawing_state.history.is_empty()
                    && self.figure_drawing_state.current_ref.is_none()
                {
                    self.figure_drawing_state.current_ref = Some(0);
                }
                return Task::future(async move {
                    info!("start loading image as reference");
                    let Ok(img) = image::open(&reference.path) else {
                        tracing::warn!("failed loading image");
                        return Message::LoadNewReference.into();
                    };
                    let img = img.to_rgba8();
                    info!("finish loading reference");
                    return Message::LoadedNewReference(reference.path, img).into();
                });
            }
            Message::LoadedNewReference(path, img) => {
                self.ref_store.ref_data.insert(path, img);
                tracing::warn!("Inserted new reference");
            }
            Message::Keypress(key_event) => {
                let keyboard::Event::KeyPressed {
                    key,
                    modified_key: _,
                    physical_key: _,
                    location: _,
                    modifiers: _,
                    text: _,
                } = key_event
                else {
                    return Task::none();
                };
                if let Some(active_page) = self.nav.active_data::<Page>() {
                    match active_page {
                        Page::FigureDrawing => match key {
                            keyboard::Key::Named(_name) => {}
                            keyboard::Key::Character(c) => {
                                info!("registered keyboard input: {c}");
                                let c = c.chars().next().unwrap();
                                if c == 'l' {
                                    return Task::done(
                                        Message::IncreaseReferenceCounter { amount: 1 }.into(),
                                    );
                                }
                                if c == 'h' {
                                    return Task::done(
                                        Message::IncreaseReferenceCounter { amount: -1 }.into(),
                                    );
                                }
                            }
                            keyboard::Key::Unidentified => {
                                tracing::warn!("unidentified keyboard press")
                            }
                        },
                        Page::ReferenceBoard => {}
                        Page::ReferenceStore => {}
                    }
                }
            }
            Message::IncreaseReferenceCounter { amount } => {
                let state = &mut self.figure_drawing_state;
                state.current_ref = match state.current_ref {
                    Some(current) => Some(current.saturating_add_signed(amount)),
                    None => Some((amount - 1).min(0) as usize),
                };
                if state.history.len() <= state.current_ref.unwrap() {
                    return Task::done(Message::LoadNewReference.into());
                };
            }
            Message::RemoveSource(source) => {
                let Some(index) = self.ref_store.source_folders.iter().position(|s| s == s) else {
                    tracing::warn!("tried to remove source {source:?} but it was not found");
                    return Task::none();
                };
                self.ref_store.source_folders.remove(index);
                self.ref_store.save_to_disk();
            }
            Message::SetSfwFilter(sfw_only) => {
                self.figure_drawing_state.sfw_only = sfw_only;
                info!("set sfw filter to {sfw_only}");
            }
            Message::SetSfwSource(is_sfw, path) => {
                if let Some(source) = self
                    .ref_store
                    .source_folders
                    .iter_mut()
                    .find(|s| s.path == path)
                {
                    source.is_sfw = is_sfw;
                } else {
                    tracing::warn!(
                        "Tried to toggle sfw flag for source. No source registered at path {path:?}"
                    );
                }
            }
        }
        Task::none()
    }

    /// Called when a nav item is selected.
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Task<Self::Message> {
        // Activate the page in the model.
        self.nav.activate(id);
        let on_enter_task: Task<Self::Message> =
            if let Some(page) = self.nav.data::<Page>(self.nav.active()) {
                match page {
                    Page::FigureDrawing => self.on_figure_drawing_enter(),
                    Page::ReferenceBoard => Task::none(),
                    Page::ReferenceStore => Task::none(),
                }
            } else {
                Task::none()
            };

        self.update_title().chain(on_enter_task)
    }
}

impl AppModel {
    pub fn on_figure_drawing_enter(&self) -> Task<<AppModel as cosmic::Application>::Message> {
        let figure_drawing_state = &self.figure_drawing_state;
        let now = Instant::now();
        if figure_drawing_state.history.is_empty()
            || figure_drawing_state.last_fetched - now > figure_drawing_state.duration_per_image
        {
            println!("send load ref message");
            Task::done(Message::LoadNewReference.into())
        } else {
            Task::none()
        }
    }
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(APP_ICON));

        let title = widget::text::title3(fl!("app_title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::OpenRepositoryUrl)
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_x(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_title(&mut self) -> Task<Message> {
        let mut window_title = fl!("app_title");

        if let Some(page) = self.nav.text(self.nav.active()) {
            window_title.push_str(" — ");
            window_title.push_str(page);
        }

        if let Some(id) = self.core.main_window_id() {
            self.set_window_title(window_title, id)
        } else {
            Task::none()
        }
    }
}

/// The page to display in the application.
#[derive(PartialEq, Debug, Clone)]
pub enum Page {
    FigureDrawing,
    ReferenceBoard,
    ReferenceStore,
}

/// The context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}
