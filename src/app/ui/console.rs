use bevy::{
    prelude::*,
    log::{self, LogPlugin},
};
use bevy_console::{self, make_layer, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsoleOpen};
use clap::{Parser, ValueEnum};

use crate::agent;
use crate::blockchain::{
    block::{AddBlockEvent, Blockchain},
    passport::Passport,
    wallet::Wallet,
};

const OPEN_BY_DEFAULT: bool = true;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bevy_console::ConsolePlugin,
        LogPlugin {
            level: log::Level::INFO,
            filter: "info,capture_bevy_logs=info".to_owned(),
            custom_layer: make_layer,
        }))
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
        ShowInfoCommandsPlugin,
        BlockchainCommandsPlugin,
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
        app.add_console_command::<ShowBotsCommand, _>(show_bots_command);
        app.add_console_command::<ShowPassportCommand, _>(show_passport_command);
        app.add_console_command::<ShowWalletCommand, _>(show_wallet_command);
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "show-bots")]
struct ShowBotsCommand;

fn show_bots_command(
    mut log: ConsoleCommand<ShowBotsCommand>,
    bots: Query<&Name, With<agent::bot::Bot>>,
) {
    if let Some(Ok(ShowBotsCommand {})) = log.take() {
        log.reply_ok(format!(
            "{}",
            bots.iter()
                .map(|name| name.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        ));
    }
}

/// Check the passport of an entity if they have one
#[derive(Parser, ConsoleCommand)]
#[command(name = "show-passport")]
struct ShowPassportCommand {
    /// Entity to check passport of
    entity_id: usize,
}

fn show_passport_command(
    mut log: ConsoleCommand<ShowPassportCommand>,
    passports: Query<&Passport>,
) {
    if let Some(Ok(ShowPassportCommand { entity_id })) = log.take() {
        let entity = Entity::from_raw(entity_id as u32);
        let passport = passports.get(entity);
        if let Ok(passport) = passport {
            log.reply_ok(format!(
                "Entity {}'s passport: {}",
                entity_id, passport.name
            ));
        } else {
            log.reply_failed(format!("Entity {} does not have a passport", entity_id));
        }
    }
}

/// Check the wallet of an entity if they have one
#[derive(Parser, ConsoleCommand)]
#[command(name = "show-wallet")]
struct ShowWalletCommand {
    /// Entity to check wallet of
    entity_id: usize,
}

fn show_wallet_command(mut log: ConsoleCommand<ShowWalletCommand>, wallets: Query<&Wallet>) {
    if let Some(Ok(ShowWalletCommand { entity_id })) = log.take() {
        let entity = Entity::from_raw(entity_id as u32);
        let wallet = wallets.get(entity);
        if let Ok(wallet) = wallet {
            log.reply_ok(format!("Entity {}'s wallet: {}", entity_id, wallet.balance));
        } else {
            log.reply_failed(format!("Entity {} does not have a wallet", entity_id));
        }
    }
}

struct BlockchainCommandsPlugin;

impl Plugin for BlockchainCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<AddBlockCommand, _>(add_block_command)
            .add_console_command::<AddBalanceCommand, _>(add_balance_command)
            .add_console_command::<CheckBalanceCommand, _>(check_balance_command);
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

/// Add a balance to an entity's wallet if they have one
#[derive(Parser, ConsoleCommand)]
#[command(name = "add-balance")]
struct AddBalanceCommand {
    /// Entity to add balance to
    entity_id: usize,
    /// Amount to add
    amount: f32,
}

fn add_balance_command(
    mut log: ConsoleCommand<AddBalanceCommand>,
    mut wallets: Query<&mut Wallet>,
) {
    if let Some(Ok(AddBalanceCommand { entity_id, amount })) = log.take() {
        // Convert usize entity_id to Entity type
        let entity = Entity::from_raw(entity_id as u32);

        if let Ok(mut wallet) = wallets.get_mut(entity) {
            wallet.balance += amount;
            log.reply_ok(format!(
                "Added {} to entity {}'s wallet. New balance: {}",
                amount, entity_id, wallet.balance
            ));
        } else {
            log.reply_failed(format!("Entity {} does not have a wallet", entity_id));
        }
    }
}

/// Check the balance of an entity's wallet if they have one
#[derive(Parser, ConsoleCommand)]
#[command(name = "check-balance")]
struct CheckBalanceCommand {
    /// Entity to check balance of
    entity_id: usize,
}

fn check_balance_command(mut log: ConsoleCommand<CheckBalanceCommand>, wallets: Query<&Wallet>) {
    if let Some(Ok(CheckBalanceCommand { entity_id })) = log.take() {
        let entity = Entity::from_raw(entity_id as u32);
        let wallet = wallets.get(entity);
        if let Ok(wallet) = wallet {
            log.reply_ok(format!(
                "Entity {}'s balance: {}",
                entity_id, wallet.balance
            ));
        } else {
            log.reply_failed(format!("Entity {} does not have a wallet", entity_id));
        }
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
