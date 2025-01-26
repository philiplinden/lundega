use bevy::prelude::*;

#[cfg(feature = "dev")]
mod debug_ui;

#[cfg(feature = "dev")]
mod console;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        #[cfg(not(feature = "dev"))]
        bevy::log::LogPlugin::default(),

        #[cfg(feature = "dev")]
        debug_ui::plugin,
    ));
}
