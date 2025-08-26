use std::sync::atomic::{AtomicU64, Ordering};

pub static PROMOTIONS: AtomicU64 = AtomicU64::new(0);
pub static DEMOTIONS: AtomicU64 = AtomicU64::new(0);

pub fn promoted() { PROMOTIONS.fetch_add(1, Ordering::Relaxed); }
pub fn demoted() { DEMOTIONS.fetch_add(1, Ordering::Relaxed); }
