/// Streams hot-tier deltas to a standby node; on a node loss the standby serves the hot cards at once.
pub struct Standby { pub lag_ms: u64 }
