// support configuring bevy lints within code
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// disable console on windows for non-dev builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

pub mod outline;
pub mod scene;

fn main() -> AppExit {
    let mut app = App::new();

    // don't panic when systems return an error!
    app.set_error_handler(bevy::ecs::error::warn);

    // ambiguous systems are bad!
    app.configure_schedules(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        ..default()
    });

    let mut default_plugins = DefaultPlugins.build();

    default_plugins = default_plugins.set(LogPlugin {
        #[cfg(feature = "dev")]
        // set the default log level to be `debug` for this crate
        filter: format!("{}=debug,{}", module_path!(), log::DEFAULT_FILTER),
        #[cfg(feature = "dev")]
        level: Level::INFO,
        ..default()
    });

    default_plugins = default_plugins.set(AssetPlugin {
        // wasm builds will check for meta files (that don't exist) if this isn't set
        // this causes errors and even panics on web build on itch
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    });

    default_plugins = default_plugins.set(WindowPlugin {
        primary_window: Window {
            fit_canvas_to_parent: true,
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            // add `bevy-dev-` prefix to the window class to use development
            // window rules such as moving the game window to a second monitor
            name: cfg_select! {
                feature = "dev" => Some(format!("bevy-dev-{}", env!("CARGO_PKG_NAME"))),
                _ => Some(env!("CARGO_PKG_NAME").into()),
            },
            ..default()
        }
        .into(),
        ..default()
    });

    app.add_plugins(default_plugins)
        .add_plugins((outline::plugin, scene::plugin));

    app.run()
}

pub(crate) mod prelude;

use crate::prelude::*;
use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    log::{self, Level, LogPlugin},
};
