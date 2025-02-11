# Redis hash per card

Date: 2025-02-11
Who: Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: One Redis hash per card with sorted-set windows reads under 50 ms at p99.
What changes: First try: rolling sums kept in Redis, one hash per card.
Setup: 3-node Redis cluster, replay of 14 Mar last year at 120k events/s.

## Result (written the same day)

Result: p99 read 140 ms at 120k events/s; the cluster CPU hit 95% on window trimming.
Learned: Trimming sorted sets on every event is the cost, not the network.
Outcome: Didn't work
