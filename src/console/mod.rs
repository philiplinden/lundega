use bevy::prelude::*;
use bevy_console::{
    reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsoleOpen, ConsolePlugin,
};
use clap::Parser;
use crate::agent::bot;
use crate::blockchain::{Block, BlockAddedEvent};

const OPEN_BY_DEFAULT: bool = true;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ConsolePlugin)
        .insert_resource(ConsoleOpen {
            open: OPEN_BY_DEFAULT,
        })
        .insert_resource(ConsoleConfiguration {
            top_pos: 0.0,
            left_pos: 5.0,
            height: 500.0,
            width: 1280.0,
            show_title_bar: false,
            ..Default::default()
        })
        .add_console_command::<SpawnBotCommand, _>(spawn_bot_command)
        .add_console_command::<AddBlockCommand, _>(add_block_command);
}

/// Spawns a new bot that prints the system time
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn-bot")]
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

/// Creates a new block and adds it to the blockchain
#[derive(Parser, ConsoleCommand)]
#[command(name = "add-block")]
struct AddBlockCommand {
    /// Data to store in the block
    data: String,
}

fn add_block_command(
    mut log: ConsoleCommand<AddBlockCommand>,
    mut block_events: EventWriter<BlockAddedEvent>,
) {
    if let Some(Ok(AddBlockCommand { data })) = log.take() {
        let block = Block::new(data);
        block_events.send(BlockAddedEvent { block: block.clone() });
        reply!(log, "Created new block: {:?}", block);
        log.ok();
    }
}
