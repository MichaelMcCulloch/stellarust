use std::{char::MAX, fmt::Display, time::SystemTime};

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
        const MAX_LEN: usize = 8;
        let empires_string = if self.empires.len() <= MAX_LEN {
            format!("{:?}", self.empires)
        } else {
            format!(
                "{:?} and {} more",
                &self.empires[0..MAX_LEN],
                self.empires.len() - MAX_LEN
            )
        };

        let last_write = self.last_write;

        write!(
            f,
            "{}\n\t{}\n\t{}",
            self.name,
            empires_string,
            OffsetDateTime::from(last_write)
        )
    }
}
