# Delta log for tier moves

Date: 2025-06-24
Who: Priya Raman
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Shipping a short delta log with each moved card keeps its history exact.
What changes: Each tier move carries the last 60 minutes of deltas.
Setup: Fixed rig, Black Friday at 200k/s.

## Result (written the same day)

Result: p99 47 ms, window error 0.0% in 2.1 million moves.
Learned: Exact windows at the target. Next: does it hold for merchant features too?
Outcome: Worked
