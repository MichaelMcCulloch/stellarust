use std::path::PathBuf;

use stellarust::dto::CampaignDto;

pub trait FileReader {
    fn read_from_path(path: &PathBuf) -> Vec<CampaignDto>;
    fn read() -> Vec<CampaignDto>;
}
