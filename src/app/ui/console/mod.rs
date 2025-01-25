mod blockchain;
mod remover;
mod spawner;

use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsoleOpen, ConsolePlugin};

const OPEN_BY_DEFAULT: bool = true;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ConsolePlugin)
        .insert_resource(ConsoleOpen {
            open: OPEN_BY_DEFAULT,
        })
        .insert_resource(ConsoleConfiguration {
            top_pos: 0.0,
            left_pos: 5.0,
            height: 500.0,
            width: 1280.0,
            show_title_bar: false,
            ..Default::default()
        });

    // Add commands plugins
    app.add_plugins((spawner::plugin, blockchain::plugin, remover::plugin));
}
