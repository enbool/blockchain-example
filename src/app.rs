
use log::{error, warn};
use sha2::{Sha256, Digest};

use crate::block::Block;

pub const DIFFICULTY_PREFIX: &str = "00";
pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
        }
    }

    pub fn genesis(&mut self) {
        let block = Block::new("hello".into(), "data".into());
        self.blocks.push(block);
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least on block");
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            error!("invalid block");
        }
    }
    // check if the block is valid
    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } 

        if !hash_to_binary_representation(&hex::decode(&block.hash).expect("cannot decode from hex"))
            .starts_with(DIFFICULTY_PREFIX) {
                warn!("block with id: {} has ivalid difficulty", block.id);
                return false;
        }

        if block.id != previous_block.id + 1 {
            warn!("block with id: {} is not the next block after the lastest: {}", block.id, previous_block.id);
            return false;
        }

        let hash = hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce,
        ));

        if hash != block.hash {
            warn!("block with id: {} has ivalid difficulty", block.id);
            return false;
        }

        return true;
    }

    fn is_chain_valid(&self, chain: &[Block]) -> bool {
        // first block needn't check
        for i in 1..chain.len() {
           let first = &chain[i - 1];
           let second = &chain[i];

           if !self.is_block_valid(first, second) {
               return false;
           }
        }

        true
    }

    /**
     * choose the longest chain
     */
    pub fn choose_chain(&mut self, local: Vec<Block>, remote: Vec<Block>) -> Vec<Block> {
        let is_local_valid = self.is_chain_valid(&local);
        let is_remote_valid = self.is_chain_valid(&remote);

        if is_local_valid && is_remote_valid {
            if local.len() > remote.len() {
                return local;
            } else {
                return remote;
            }
        }
        if is_local_valid {
            return local;
        } 
        if is_remote_valid {
            return remote;
        }
        panic!("local and remote chains are both invalid");
    }
}

pub fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();

    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({
        "id": id, 
        "previous_hash": previous_hash,
        "data": data,
        "timestamp": timestamp,
        "nonce": nonce
    });

    let mut hasher = Sha256::default();

    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}