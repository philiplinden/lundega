use std::time::{SystemTime, UNIX_EPOCH};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, bot_print_time);
}

#[derive(Component)]
pub struct Bot {
    last_print: f32,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            last_print: 0.0,
        }
    }
}

pub fn spawn_bot(commands: &mut Commands) -> Entity {
    commands.spawn((
        Bot::default(),
    )).id()
}

pub fn bot_print_time(
    time: Res<Time>,
    mut query: Query<&mut Bot>,
) {
    for mut bot in query.iter_mut() {
        if time.elapsed_secs() >= bot.last_print + 1.0 {
            if let Ok(current_time) = SystemTime::now().duration_since(UNIX_EPOCH) {
                info!("Current system time: {:?}", current_time);
                bot.last_print = time.elapsed_secs();
            }
        }
    }
}
