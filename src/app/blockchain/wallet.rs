use bevy::prelude::*;
use uuid::Uuid;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Wallet>();
}

#[derive(Component, Reflect, Default)]
pub struct Wallet {
    pub address: String,
    pub balance: f32,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            address: Uuid::new_v4().to_string(),
            balance: 0.0,
        }
    }

    pub fn with_balance(mut self, balance: f32) -> Self {
        self.balance = balance;
        self
    }
}
