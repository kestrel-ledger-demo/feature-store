# Plan: per-card features at p99 under 50 ms

Written: 2025-01-08, by Dana Okafor, at the start of the project and before the first experiment.

## What we don't know
Whether per-card rolling features (spend in the last 1, 10 and 60 minutes, new-merchant counts) can be kept
exact and current, and read at p99 under 50 ms, at 200,000 events per second.

## What is known, and why it isn't enough
- Standard practice: one Redis hash per card with sorted-set windows, trimmed on every event.
- The feature stores we are reviewing refresh windows in batches (seconds of lag) or document p99 reads
  above 50 ms at over 1 million keys. None publishes results at our card count and event rate.
- Approximate windows (fixed buckets) are cheaper, but the model is sensitive to window error.

## How we will test it
- A replay rig that plays recorded processor days against the store; p99 read latency in an HDR histogram.
- H1: a Redis hash per card with sorted sets reads under 50 ms at p99.
- H2, if trimming is the cost: fixed 10 s buckets.
- H3: sharding the store so no shard is hot.
- Done when p99 is under 50 ms at 200,000 events per second with exact 1, 10 and 60 minute windows.
- Each run gets a note in docs/experiments/ with the plan before it starts and the result the same day.
