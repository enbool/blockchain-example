use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Release;

use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub noce: u64,
}

impl Block {
    pub fn new(data: String) -> Self {
        let block = Self { 
            id: ID_KEEPER.fetch_add(1, Release), 
            hash: "0".into(), 
            previous_hash: "0".into(), 
            timestamp: Utc::now().timestamp(), 
            data: data, 
            noce: 123, 
        };
        block.hash = "new hash".into();

        block
    }
}

static ID_KEEPER: AtomicU64 = AtomicU64::new(0) ;

