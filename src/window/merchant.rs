/// New-merchant counts on the same two tiers (no more cross-shard reads).
pub fn new_merchants_last_hour(seen: &std::collections::HashSet<String>, recent: &[String]) -> usize { recent.iter().filter(|m| !seen.contains(*m)).count() }
