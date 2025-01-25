use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::blockchain::AddBlockEvent;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, bot_print_time);
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

pub fn bot_print_time(
    mut events: EventWriter<AddBlockEvent>,
    time: Res<Time>,
    mut query: Query<(&Name, &mut Bot)>,
) {
    for (name, mut bot) in query.iter_mut() {
        if time.elapsed_secs() >= bot.last_print + 5.0 {
            if let Ok(current_time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                bot.message = format!("[Bot {}] Current system time: {:?}", name, current_time);
                bot.last_print = time.elapsed_secs();
                events.send(AddBlockEvent {
                    timestamp: time.elapsed_secs(),
                    data: bot.message.clone(),
                });
            }
        }
    }
}
