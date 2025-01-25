pub mod agent;
pub mod blockchain;
pub mod registry;
pub mod ui;
pub mod console;

use bevy::{
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
        app.add_plugins((DefaultPlugins
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
            }),));

        // Custom plugins
        app.add_plugins((SimulationPlugin, UiPlugin));
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

/// Plugin for the simulation actions and logic
struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((agent::plugin, blockchain::plugin, registry::plugin));
    }
}

/// Plugin for the UI and console
struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui::plugin, console::plugin));
    }
}
