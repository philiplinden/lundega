#![allow(unused)]

use bevy::prelude::*;

use crate::blockchain::AddBlockEvent;

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
