use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Release;

use chrono::Utc;
use log::info;
use serde::{Serialize, Deserialize};

use crate::app;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(previous_hash: String, data: String) -> Self {
        let now = Utc::now().timestamp();
        let id = ID_KEEPER.fetch_add(1, Release);
        let (nonce, hash) = mine_block(id, now, &previous_hash, &data);
        let block = Self { 
            id, 
            hash, 
            previous_hash, 
            timestamp: now, 
            data, 
            nonce, 
        };

        block
    }
}

static ID_KEEPER: AtomicU64 = AtomicU64::new(0);

fn mine_block(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
    info!("mining block...");

    let mut nonce = 0;
    loop {
        if nonce % 100000 == 0 {
            info!("nonce: {}", nonce);
        }
        let hash = app::calculate_hash(id, timestamp, previous_hash, data, nonce);
        let binary_hash = app::hash_to_binary_representation(&hash);

        if binary_hash.starts_with(app::DIFFICULTY_PREFIX) {
            info!("mined! nonce: {}, hash: {}, binary hash: {}",
                nonce,
                hex::encode(&hash),
                binary_hash
            );
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

