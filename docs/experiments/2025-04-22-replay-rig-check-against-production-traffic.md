# Replay rig check against production traffic

Date: 2025-04-22
Who: Lea Fontaine
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: The replay rig's latency matches what production saw on the same day within 5%.
What changes: Replay rig compared to production logs for 3 recorded days.
Setup: Replay of 3 days, same hardware class as production.

## Result (written the same day)

Result: Rig was 11% optimistic: it replayed without TLS and without the processor's burst pattern.
Learned: Add TLS and burst timing to the rig before trusting any number from it.
Outcome: Didn't work
