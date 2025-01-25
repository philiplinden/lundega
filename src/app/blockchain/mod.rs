pub mod consensus;
pub mod passport;
pub mod transaction;
pub mod wallet;
pub mod block;

use bevy::prelude::*;

pub use passport::Passport;
pub use wallet::Wallet;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins((
        block::plugin,
        consensus::plugin,
        wallet::plugin,
        transaction::plugin,
        passport::plugin,
    ));
}
