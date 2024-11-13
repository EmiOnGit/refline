use tracing::Level;
use tracing_subscriber::{
    filter::{FilterFn, LevelFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
const LOG_IGNORE: [&str; 12] = [
    "vulkan::instance",
    "wgpu_core::instance",
    "wgpu_core::resource",
    "wgpu_core::device",
    "wgpu_core::present",
    "wgpu_hal",
    "iced_wgpu::image",
    "iced_wgpu::backend",
    "cosmic_text::font",
    "cosmic_text::buffer",
    "naga",
    "sctk",
];
const LOG_IGNORE_ALL: [&str; 1] = ["cosmic_config::dbus"];
pub fn logger_init(debug: bool) {
    let filter = FilterFn::new(move |meta| {
        let target = meta.target();
        let in_ignore_list = LOG_IGNORE.iter().any(|name| target.contains(name));
        let in_ignoreall_list = LOG_IGNORE_ALL.iter().any(|name| target.contains(name));
        if !debug && target.contains("refline") && target.contains("debug") {
            return false;
        }
        if in_ignore_list {
            meta.level() < &Level::WARN
        } else if in_ignoreall_list {
            false
        } else {
            true
        }
    })
    .with_max_level_hint(if debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    });

    let fmt_layer = tracing_subscriber::fmt::layer().with_target(true).pretty();
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
}
