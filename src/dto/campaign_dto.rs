use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CampaignDto {
    pub name: String,
    pub empires: Vec<String>,
    pub last_write: OffsetDateTime,
}
