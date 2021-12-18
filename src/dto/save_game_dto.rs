use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
pub struct SaveGameDto {
    pub empires: Vec<String>,
    pub last_save_zoned_date_time: OffsetDateTime,
}
