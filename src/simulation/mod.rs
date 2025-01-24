pub mod agent;
pub mod resource;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        agent::plugin,
    ));
}
