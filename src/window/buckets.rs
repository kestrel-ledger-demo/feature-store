/// TRIAL: fixed 10 s buckets per card instead of sorted sets (no per-event trim).
pub struct Buckets { pub slots: [i64; 360], pub head_ts: i64 }

impl Buckets {
    pub fn add(&mut self, ts_ms: i64, amount_cents: i64) { self.slots[((ts_ms / 10_000) % 360) as usize] += amount_cents; self.head_ts = ts_ms; }
    pub fn sum_last(&self, minutes: usize) -> i64 { let n = minutes * 6; (0..n).map(|i| self.slots[(((self.head_ts / 10_000) as usize + 360 - i) % 360)]).sum() }
}

/// Worst relative error of the 1 minute sum vs an exact window, on a random stream (used by the property test).
pub fn max_edge_error(events: usize, seed: u64) -> f64 {
    let (mut x, mut b, mut exact, mut worst) = (seed, Buckets { slots: [0; 360], head_ts: 0 }, Vec::new(), 0.0f64);
    for i in 0..events {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let ts = i as i64 * 700 + (x >> 54) as i64; let amt = 100 + (x >> 50) as i64 % 5_000;
        b.add(ts, amt); exact.push((ts, amt));
        let want: i64 = exact.iter().filter(|(t, _)| *t > ts - 60_000).map(|(_, a)| a).sum();
        if want > 0 { worst = worst.max(((b.sum_last(1) - want).abs() as f64) / want as f64); }
    }
    worst
}
