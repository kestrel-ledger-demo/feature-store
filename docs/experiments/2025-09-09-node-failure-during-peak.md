# Node failure during peak

Date: 2025-09-09
Who: Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Losing one node at peak keeps p99 under 50 ms while hot cards rebuild.
What changes: One of 6 nodes killed at the peak minute.
Setup: Fixed rig, Black Friday at 200k/s.

## Result (written the same day)

Result: p99 rose to 88 ms for 4 minutes while 30,000 hot cards rebuilt from Redis.
Learned: Recovery is the open question: hot state needs a warm copy somewhere.
Outcome: Partly worked
