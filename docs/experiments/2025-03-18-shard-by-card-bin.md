# Shard by card BIN

Date: 2025-03-18
Who: Owen Tremblay, Priya Raman
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Sharding by the first 6 digits spreads cards so no shard is hot.
What changes: Keys sharded by BIN across 12 shards.
Setup: Replay of the Black Friday day, scaled to 200k events/s.

## Result (written the same day)

Result: Median 18 ms but p99 70 ms: 3 large issuers' BINs made 3 shards hot.
Learned: Card traffic is not spread by BIN: a few issuers carry most of it.
Outcome: Partly worked
