/// TRIAL: shard keys by the card's BIN (first 6 digits) on a consistent-hash ring of 12 shards.
pub fn shard_of(bin: &str, ring: &[(u64, u32)]) -> u32 {
    let h = fxhash(bin.as_bytes());
    ring.iter().find(|(k, _)| *k >= h).or(ring.first()).map(|(_, s)| *s).unwrap_or(0)
}
fn fxhash(b: &[u8]) -> u64 { b.iter().fold(0xcbf29ce484222325, |h, x| (h ^ *x as u64).wrapping_mul(0x100000001b3)) }
