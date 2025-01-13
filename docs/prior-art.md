# Prior-art search: exact per-card rolling windows at 200,000 events per second

Written: 2025-01-13, by Owen Tremblay, before the first experiment.

## What we searched
- Redis documentation: sorted sets (ZADD, ZREMRANGEBYSCORE complexity) and RedisTimeSeries aggregations.
- Feast and Hopsworks documentation: how online features are computed and served.
- Google Scholar: "streaming feature store p99 latency", "sliding window aggregation per key", "exact sliding window stream processing".

## What we found
- Exact windows: one sorted set per key, trimmed on every event. Trim cost grows with events per key.
- Cheaper windows: fixed time buckets, which are approximate at the window edge.
- Feast and Hopsworks serve features from an online key-value store; window aggregations are computed by a
  separate batch or stream job, so a window is only as fresh as that job's last run.
- Spreading load: sharding keys on a consistent-hash ring.

## What we did not find
- Any published p99 read under 50 ms at 200,000 events per second with exact 1, 10 and 60 minute windows per card.
- Any method that keeps a key's window exact when the key moves between storage tiers.
