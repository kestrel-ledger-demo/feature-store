use std::collections::HashMap;

/// TRIAL: the busiest cards keep exact windows in process memory; the rest stay in Redis.
pub struct HotTier { pub cards: HashMap<String, crate::window::buckets::Buckets>, pub capacity: usize }
