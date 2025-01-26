#[cfg(feature = "inspect")]
mod inspect {
    use bevy::prelude::*;
    use bevy_inspector_egui::prelude::*;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    pub(super) fn plugin(app: &mut App) {
        // Toggle the debug overlay for UI.
        app.add_plugins(WorldInspectorPlugin::new());
    }
}

use bevy::{
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DebugUiPlugin,

        #[cfg(not(feature = "inspect"))]
        super::console::plugin,

        #[cfg(feature = "inspect")]
        inspect::plugin,
    )).add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::F4;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
