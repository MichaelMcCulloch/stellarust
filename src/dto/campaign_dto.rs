use std::{fmt::Display, time::SystemTime};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct CampaignDto {
    pub name: String,
    pub empires: Vec<String>,
    pub last_write: SystemTime,
}

impl Display for CampaignDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date: DateTime<Utc> = self.last_write.into();
        write!(
            f,
            "{}\n\t{}\n\t{}",
            self.name,
            format!("{:?}", self.empires),
            date
        )
    }
}
