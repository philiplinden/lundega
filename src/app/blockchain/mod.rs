pub mod consensus;
pub mod passport;
pub mod transaction;
pub mod wallet;
mod block;

use bevy::prelude::*;

pub(crate) use block::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        block::plugin,
        consensus::plugin,
        wallet::plugin,
        transaction::plugin,
        passport::plugin,
    ));
}
