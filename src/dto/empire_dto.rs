use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EmpireDto {
    pub name: String,
}
