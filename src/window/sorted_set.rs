use redis::AsyncCommands;

/// TRIAL: one sorted set per card and window; every event adds a member and trims old ones.
pub async fn add(con: &mut redis::aio::MultiplexedConnection, card: &str, ts_ms: i64, amount_cents: i64) -> redis::RedisResult<()> {
    for (w, span) in [("1m", 60_000), ("10m", 600_000), ("60m", 3_600_000)] {
        let key = format!("w:{w}:{card}");
        let _: () = con.zadd(&key, format!("{ts_ms}:{amount_cents}"), ts_ms).await?;
        let _: () = con.zrembyscore(&key, 0, ts_ms - span).await?; // the trim on every event
    }
    Ok(())
}
