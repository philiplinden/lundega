use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::blockchain::{Block, BlockAddedEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (bot_print_time, add_blocks_from_bots).chain());
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

pub fn bot_print_time(time: Res<Time>, mut query: Query<&mut Bot>) {
    for mut bot in query.iter_mut() {
        if time.elapsed_secs() >= bot.last_print + 1.0 {
            if let Ok(current_time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                bot.message = format!("Current system time: {:?}", current_time);
                bot.last_print = time.elapsed_secs();
            }
        }
    }
}

fn add_blocks_from_bots(mut events: EventWriter<BlockAddedEvent>, mut query: Query<&mut Bot>) {
    for bot in query.iter_mut() {
        let new_block = Block::new(bot.message.clone());
        events.send(BlockAddedEvent { block: new_block });
    }
}
