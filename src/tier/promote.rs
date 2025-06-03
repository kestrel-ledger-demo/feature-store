/// Promote a card to the hot tier when it passes `rate_per_min`; demote after 10 quiet minutes.
pub fn should_promote(requests_last_min: u32, rate_per_min: u32) -> bool { requests_last_min >= rate_per_min }
pub fn should_demote(quiet_ms: i64) -> bool { quiet_ms > 600_000 }
