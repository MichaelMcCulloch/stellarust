use serde::{Deserialize, Serialize};

use super::CampaignDto;

#[derive(Deserialize, Serialize)]
pub struct EmpireDto {
    pub name: String,
}
