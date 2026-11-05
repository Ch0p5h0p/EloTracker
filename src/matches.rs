use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Match {
    pub player_a: u64,
    pub player_b: u64,
    pub score_a: f64,
    pub score_b: f64,

    pub adjustment_a: f64,
    pub adjustment_b: f64,

    pub timestamp: i64
}

impl Match {
    pub fn new(player_a: u64, player_b: u64, scores: Vec<f64>) -> Self {
        let timestamp = Utc::now().timestamp();
        Match {
            player_a,
            player_b,
            score_a: *scores.get(0).unwrap(),
            score_b: *scores.get(1).unwrap(),

            adjustment_a: 0.0,
            adjustment_b: 0.0,

            timestamp,
        }
    }
}
