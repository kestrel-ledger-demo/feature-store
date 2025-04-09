use std::hash::{Hash, Hasher};

/// Shard by a hash of the card token: spreads issuers evenly, unlike BIN prefixes.
pub fn shard_of(card_token: &str, shards: u32) -> u32 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    card_token.hash(&mut h);
    (h.finish() % shards as u64) as u32
}
