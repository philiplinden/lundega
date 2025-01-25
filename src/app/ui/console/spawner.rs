use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use clap::Parser;

use crate::agent::bot;

pub(super) fn plugin(app: &mut App) {
    app.add_console_command::<SpawnBotCommand, _>(spawn_bot_command);
}

/// Spawns a new bot that prints the system time
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn-bot")]
struct SpawnBotCommand {
    /// Name of the bot
    name: Option<String>,
}

fn spawn_bot_command(mut log: ConsoleCommand<SpawnBotCommand>, mut commands: Commands) {
    if let Some(Ok(SpawnBotCommand { name })) = log.take() {
        let entity = bot::spawn_bot(&mut commands);
        if let Some(name) = name {
            commands.entity(entity).insert(Name::new(name));
        } else {
            commands.entity(entity).insert(Name::new(entity.index().to_string()));
        }
        reply!(log, "Spawned new bot with entity id: {:?}", entity);
        log.ok();
    }
}
