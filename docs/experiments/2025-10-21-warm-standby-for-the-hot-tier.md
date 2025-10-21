# Warm standby for the hot tier

Date: 2025-10-21
Who: Owen Tremblay, Priya Raman
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Streaming hot-tier deltas to a standby keeps p99 under 50 ms through a node loss.
What changes: Hot tier deltas streamed to a standby node.
Setup: Fixed rig, one node killed at peak.
