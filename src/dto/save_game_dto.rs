use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct SaveGameDto {
    pub save_name: String,
    pub empires: Vec<String>,
    pub last_save_zoned_date_time: OffsetDateTime,
}
