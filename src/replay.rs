use crate::event::AuthEvent;
use std::io::BufRead;

/// Reads a recorded day (one JSON event per line) for load tests.
pub fn load_day(path: &str) -> std::io::Result<Vec<AuthEvent>> {
    let f = std::io::BufReader::new(std::fs::File::open(path)?);
    Ok(f.lines().filter_map(|l| l.ok()).filter_map(|l| serde_json::from_str(&l).ok()).collect())
}

/// Gaps between events as recorded (ms), so a replay keeps the processor's bursts.
pub fn inter_arrival_ms(events: &[AuthEvent]) -> Vec<i64> {
    events.windows(2).map(|w| (w[1].ts_ms - w[0].ts_ms).max(0)).collect()
}
