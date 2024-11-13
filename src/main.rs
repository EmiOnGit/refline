// SPDX-License-Identifier: {{LICENSE}}

mod app;
mod config;
mod figure_drawing;
mod i18n;
mod log;
mod reference;
mod view;

fn main() -> cosmic::iced::Result {
    // start logging
    log::logger_init(false);
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default();

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::AppModel>(settings, ())
}
