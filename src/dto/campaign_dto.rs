use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CampaignDto {
    pub name: String,
    pub empires: Vec<String>,
    pub last_write: SystemTime,
}
