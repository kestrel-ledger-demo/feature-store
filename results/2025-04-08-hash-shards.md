# Hash sharding by card token

Black Friday replay at 200k events/s. Shards even (max 1.04x mean).
p99 61 ms: merchant counts need cross-shard reads (+20 ms). Per-card sharding alone can't reach 50 ms.
