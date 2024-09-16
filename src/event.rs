use serde::{Deserialize, Serialize};

/// One card authorization as the processor sends it (no PAN: a token per card).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthEvent {
    pub card_token: String,
    pub merchant_id: String,
    pub amount_cents: i64,
    pub ts_ms: i64,
}
