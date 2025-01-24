mod camera;
mod console;
mod debug_ui;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, console::plugin, debug_ui::plugin));
}
