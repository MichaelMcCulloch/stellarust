use std::{fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CampaignDto {
    pub name: String,
    pub empires: Vec<String>,
    pub last_write: SystemTime,
}

impl Display for CampaignDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\t{}\n\t{}",
            self.name,
            format!("{:?}", self.empires),
            OffsetDateTime::from(self.last_write)
        )
    }
}
