use bevy::prelude::*;
use bevy_console::{
    AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsoleOpen, ConsolePlugin,
};
use clap::{Parser, ValueEnum};

use crate::agent;
use crate::blockchain::{AddBlockEvent, Blockchain};

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
        });

    // Add commands plugins
    app.add_plugins((
        SpawnCommandsPlugin,
        BlockchainCommandsPlugin,
        ShowInfoCommandsPlugin,
    ));
}

struct SpawnCommandsPlugin;

impl Plugin for SpawnCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SpawnCommand, _>(spawn_command)
            .add_console_command::<DespawnCommand, _>(despawn_command);
    }
}

#[derive(Clone, ValueEnum, Debug)]
enum EntityType {
    Bot,
}

/// Spawns an entity of the specified type
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn")]
struct SpawnCommand {
    /// Type of entity to spawn
    #[arg(value_enum)]
    entity_type: EntityType,
    /// Optional name for the entity
    #[arg(short, long)]
    name: Option<String>,
}

fn spawn_command(mut log: ConsoleCommand<SpawnCommand>, mut commands: Commands) {
    if let Some(Ok(SpawnCommand { entity_type, name })) = log.take() {
        match entity_type {
            EntityType::Bot => {
                let entity = agent::bot::spawn_bot(&mut commands);
                if let Some(name) = name {
                    commands.entity(entity).insert(Name::new(name));
                } else {
                    commands
                        .entity(entity)
                        .insert(Name::new(entity.index().to_string()));
                }
                log.reply_ok(format!("Spawned new bot with entity id: {:?}", entity));
            }
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "despawn")]
struct DespawnCommand {
    /// Entity to remove
    entity_id: u32,
}

fn despawn_command(mut log: ConsoleCommand<DespawnCommand>, mut commands: Commands) {
    if let Some(Ok(DespawnCommand { entity_id })) = log.take() {
        commands
            .entity(Entity::from_raw(entity_id))
            .despawn_recursive();
        log.ok();
    }
}

struct ShowInfoCommandsPlugin;

impl Plugin for ShowInfoCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<ShowChainCommand, _>(show_chain_command);
        app.add_console_command::<AllBotsCommand, _>(all_bots_command);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "all-bots")]
struct AllBotsCommand;

fn all_bots_command(mut log: ConsoleCommand<AllBotsCommand>, bots: Query<&Name, With<agent::bot::Bot>>) {
    if let Some(Ok(AllBotsCommand {})) = log.take() {
        log.reply_ok(format!(
            "{}",
            bots.iter()
                .map(|name| name.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }
}

struct BlockchainCommandsPlugin;

impl Plugin for BlockchainCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<AddBlockCommand, _>(add_block_command);
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
    mut block_events: EventWriter<AddBlockEvent>,
    time: Res<Time>,
) {
    if let Some(Ok(AddBlockCommand { data })) = log.take() {
        block_events.send(AddBlockEvent {
            timestamp: time.elapsed_secs(),
            data,
        });
        log.ok();
    }
}

/// Prints the current blockchain
#[derive(Parser, ConsoleCommand)]
#[command(name = "show-chain")]
struct ShowChainCommand;

fn show_chain_command(mut log: ConsoleCommand<ShowChainCommand>, blockchain: Res<Blockchain>) {
    if let Some(Ok(_)) = log.take() {
        for block in blockchain.chain.iter() {
            log.reply(format!("{}", block));
        }
        log.ok();
    }
}
