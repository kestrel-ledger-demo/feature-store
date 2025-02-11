# Redis sorted sets, one per card per window

3-node Redis cluster, replay of last year's 14 March at 120k events/s.
p99 read 140 ms. Cluster CPU 95%, almost all in ZREMRANGEBYSCORE (the trim on every event).
