# Shard by card hash

Date: 2025-04-08
Who: Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Hashing the full card number removes hot shards.
What changes: BIN key replaced with a hash of the card token.
Setup: Black Friday replay at 200k/s.
