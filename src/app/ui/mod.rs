mod console;
mod debug_ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        console::plugin,
        debug_ui::plugin,
    ));
}
