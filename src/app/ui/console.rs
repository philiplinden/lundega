use bevy::prelude::*;
use bevy_console::{
    reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsoleOpen, ConsolePlugin,
};
use clap::Parser;
use crate::simulation::agent::bot;

const OPEN_BY_DEFAULT: bool = true;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ConsolePlugin)
        .insert_resource(ConsoleOpen {
            open: OPEN_BY_DEFAULT,
        })
        .insert_resource(ConsoleConfiguration {
            top_pos: 0.0,
            left_pos: 0.0,
            height: 720.0,
            width: 1280.0,
            show_title_bar: false,
            ..Default::default()
        })
        .add_console_command::<TestCommand, _>(test_command)
        .add_console_command::<SpawnBotCommand, _>(spawn_bot_command)
        .add_systems(Startup, || {
            log::info!("Hi!");
            log::warn!("This is a warning!");
            log::debug!("You won't see me!");
            log::error!("This is an error!");
            log::info!("Bye!");
        });
}

/// Prints given arguments to the console
/// (this is a placeholder for a real command)
#[derive(Parser, ConsoleCommand)]
#[command(name = "test")]
struct TestCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

/// (this is a placeholder for a real command)
fn test_command(mut log: ConsoleCommand<TestCommand>) {
    if let Some(Ok(TestCommand { msg, num })) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}

/// Spawns a new bot that prints the system time
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn_bot")]
struct SpawnBotCommand;

fn spawn_bot_command(
    mut log: ConsoleCommand<SpawnBotCommand>,
    mut commands: Commands,
) {
    if let Some(Ok(_)) = log.take() {
        let entity = bot::spawn_bot(&mut commands);
        reply!(log, "Spawned new bot with entity id: {:?}", entity);
        log.ok();
    }
}
