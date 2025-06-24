/// TRIAL: every tier move carries the card's last 60 minutes of deltas, so its windows stay exact.
#[derive(Default, Clone)]
pub struct DeltaLog { pub deltas: Vec<(i64, i64)> } // (ts_ms, amount_cents)

impl DeltaLog {
    pub fn push(&mut self, ts_ms: i64, amount: i64) { self.deltas.push((ts_ms, amount)); self.deltas.retain(|(t, _)| *t > ts_ms - 3_600_000); }
}
