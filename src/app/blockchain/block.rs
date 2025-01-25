use std::fmt::Display;

use bevy::prelude::*;
use sha2::{Digest, Sha256};
use hex;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Blockchain>()
        .add_event::<AddBlockEvent>()
        .add_systems(Update, handle_add_block_event)
        .add_systems(Startup, create_genesis_block);
}

#[derive(Resource, Default)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, timestamp: f32, data: String) {
        let block_data = BlockConstructor::new(timestamp, data);
        let last_block = self.last_block();
        let new_block = block_data.build(last_block.index + 1, last_block.hash.clone());
        self.chain.push(new_block);
    }
}

#[derive(Clone, Debug, Default)]
pub struct Block {
    pub index: usize,
    pub timestamp: f32,
    pub data: String,
    pub previous_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block #{}: Hash = {}, Previous Hash = {}, Timestamp = {}, Data = {}",
            self.index,
            hex::encode(&self.hash),
            hex::encode(&self.previous_hash),
            self.timestamp,
            self.data
        )
    }
}

pub struct BlockConstructor {
    timestamp: f32,
    data: String,
}

impl BlockConstructor {
    pub fn new(timestamp: f32, data: String) -> Self {
        Self { timestamp, data }
    }

    pub fn build(self, index: usize, previous_hash: Vec<u8>) -> Block {
        let timestamp = self.timestamp;
        let data = self.data;
        let hash = calculate_block_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

// Calculate block hash using SHA-256
fn calculate_block_hash(index: usize, timestamp: f32, data: &str, previous_hash: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", index, timestamp, data));
    hasher.update(previous_hash);
    hasher.finalize().to_vec()
}

/// System to create the initial genesis block
fn create_genesis_block(mut commands: Commands, time: Res<Time>) {
    let genesis_block = Block {
        index: 0,
        timestamp: time.elapsed_secs(),
        data: "Genesis Block".to_string(),
        previous_hash: vec![0],
        hash: calculate_block_hash(0, 0.0, "Genesis Block", &[0]),
    };
    commands.insert_resource(Blockchain {
        chain: vec![genesis_block],
    });
}

#[derive(Event, Debug, Default)]
pub struct AddBlockEvent {
    pub timestamp: f32,
    pub data: String,
}

/// Add a new block to the blockchain
fn handle_add_block_event(
    mut events: EventReader<AddBlockEvent>,
    mut blockchain: ResMut<Blockchain>,
) {
    for event in events.read() {
        info!("Adding block to blockchain: {}", event.data);
        blockchain.add_block(event.timestamp, event.data.clone());
    }
}
