use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub rating: f64,
}
