# Merchant features on the same tiers

Date: 2025-07-15
Who: Priya Raman, Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Merchant counts on the same two tiers keep p99 under 50 ms.
What changes: New-merchant counts moved from cross-shard reads to the tiered store.
Setup: Fixed rig, Black Friday at 200k/s.

## Result (written the same day)

Result: p99 43 ms with all features; memory per node up 30%.
Learned: Target met at 200k/s. Memory cost is acceptable on the current node size.
Outcome: Worked
