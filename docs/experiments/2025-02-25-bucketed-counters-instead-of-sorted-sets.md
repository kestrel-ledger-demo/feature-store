# Bucketed counters instead of sorted sets

Date: 2025-02-25
Who: Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Fixed 10-second buckets avoid the trimming cost.
What changes: Sorted sets replaced with 10 s bucket counters per card.
Setup: Same cluster and replay day.
