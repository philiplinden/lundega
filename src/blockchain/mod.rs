pub mod passport;
pub mod ledger;
pub mod transaction;
pub mod wallet;
pub mod consensus;

use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<Blockchain>()
       .add_event::<BlockAddedEvent>()
       .add_systems(Update, handle_block_added_event);
}

#[derive(Resource, Default)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Block {
    pub data: String,
    pub timestamp: f32,
}

impl Block {
    pub fn new(data: String, timestamp: f32) -> Self {
        Self { data, timestamp }
    }
}

#[derive(Event, Debug, Default)]
pub struct BlockAddedEvent {
    pub block: Block,
}

fn handle_block_added_event(
    mut events: EventReader<BlockAddedEvent>,
    mut blockchain: ResMut<Blockchain>,
) {
    for event in events.read() {
        blockchain.blocks.push(event.block.clone());
        info!("Block added: {:?}", event.block);
    }
}
