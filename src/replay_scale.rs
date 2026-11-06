use crate::event::AuthEvent;

/// Scales a recorded day up by replaying each event `factor` times with fresh card tokens.
pub fn scale(events: &[AuthEvent], factor: f64) -> Vec<AuthEvent> {
    let n = factor.floor() as usize;
    let mut out = Vec::with_capacity(events.len() * n);
    for i in 0..n { for e in events { let mut c = e.clone(); c.card_token = format!("{}-{i}", e.card_token); out.push(c); } }
    out
}
