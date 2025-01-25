pub mod bot;
pub mod factory;
pub mod handshake;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((bot::plugin, factory::plugin, handshake::plugin));
}
