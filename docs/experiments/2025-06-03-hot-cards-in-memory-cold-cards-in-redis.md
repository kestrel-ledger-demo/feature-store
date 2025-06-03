# Hot cards in memory, cold cards in Redis

Date: 2025-06-03
Who: Priya Raman, Owen Tremblay
Uncertainty: U1 Fresh features at p99 under 50 ms

## Plan (written before the run)

Hypothesis: Keeping the busiest 2% of cards in process memory removes the tail.
What changes: Two tiers: in-memory windows for hot cards, Redis for the rest.
Setup: Fixed rig, Black Friday at 200k/s.

## Result (written the same day)

Result: p99 52 ms, but cards moving between tiers lost up to 40 s of history.
Learned: Tier moves need the window state to travel with the card.
Outcome: Partly worked
