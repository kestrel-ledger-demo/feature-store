/// Rolling sums per card over fixed windows.
pub trait Windows {
    fn add(&mut self, card: &str, ts_ms: i64, amount_cents: i64);
    fn sums(&self, card: &str, now_ms: i64) -> [i64; 3]; // 1, 10, 60 minutes
}
