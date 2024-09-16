# feature-store

Per-card rolling features for real-time fraud scoring: spend in the last 1, 10 and 60 minutes, count of new
merchants, distance from the card's usual merchants. Read on every authorization by scoring-engine.

Target (see design doc): p99 read under 50 ms at 200,000 events per second.
