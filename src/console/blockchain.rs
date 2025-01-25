use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand};
use clap::Parser;

use crate::blockchain::{Block, BlockAddedEvent, Blockchain};

pub(super) fn plugin(app: &mut App) {
    app.add_console_command::<AddBlockCommand, _>(add_block_command)
       .add_console_command::<PrintChainCommand, _>(print_chain_command);
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
    time: Res<Time>,
) {
    if let Some(Ok(AddBlockCommand { data })) = log.take() {
        let block = Block::new(data, time.elapsed_secs());
        block_events.send(BlockAddedEvent {
            block: block.clone(),
        });
        reply!(log, "Created new block: {:?}", block);
        log.ok();
    }
}

/// Prints the current blockchain
#[derive(Parser, ConsoleCommand)]
#[command(name = "print-chain")]
struct PrintChainCommand;

fn print_chain_command(
    mut log: ConsoleCommand<PrintChainCommand>,
    blockchain: Res<Blockchain>,
) {
    if let Some(Ok(_)) = log.take() {
        reply!(log, "Current blockchain: {:?}", blockchain.blocks);
        log.ok();
    }
}
