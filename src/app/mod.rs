pub mod agent;
pub mod blockchain;
pub mod registry;
pub mod ui;
use bevy::{
    app::PluginGroupBuilder,
    log::{self, LogPlugin},
    prelude::*,
};
use bevy_console::make_layer;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        );

        // Default plugins
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "🏪 L U N D E G A 🌙".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(LogPlugin {
                    level: log::Level::INFO,
                    filter: "info,capture_bevy_logs=info".to_owned(),
                    custom_layer: make_layer,
                }),
            SimulationPlugins,
            ui::plugin,
        ));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

pub struct SimulationPlugins;

impl PluginGroup for SimulationPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(agent::plugin)
            .add(blockchain::plugin)
            .add(registry::plugin)
    }
}
