use std::time::Instant;

use bevy::prelude::*;
use uuid::Uuid;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<Passport>();
}

#[derive(Component, Reflect)]
pub struct Passport {
    pub name: String,
    pub address: String,
    pub expiry: Instant,
}

impl Passport {
    pub fn new() -> Self {
        Self {
            name: Uuid::new_v4().to_string(),
            address: Uuid::new_v4().to_string(),
            expiry: Instant::now(),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_expiry(mut self, expiry: Instant) -> Self {
        self.expiry = expiry;
        self
    }
}
