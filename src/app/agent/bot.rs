#![allow(unused)]

use bevy::prelude::*;

use crate::blockchain::Passport;
use crate::blockchain::Wallet;

pub(super) fn plugin(app: &mut App) {
    // todo
}

#[derive(Component)]
pub struct Bot {
    message: String,
    last_print: f32,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            last_print: 0.0,
        }
    }
}

pub fn spawn_bot(commands: &mut Commands) -> Entity {
    commands.spawn((Bot::default(),)).id()
}

fn spawn_wallet_on_add(mut commands: Commands, query: Query<Entity, Added<Bot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Wallet::new());
    }
}

fn spawn_passport_on_add(mut commands: Commands, query: Query<Entity, Added<Bot>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Passport::new());
    }
}
