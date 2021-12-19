use anyhow::Result;
use std::path::PathBuf;
use stellarust::dto::CampaignDto;

pub struct ModelCustodian {
    pub directory: PathBuf,
}

impl ModelCustodian {
    pub fn create(campaign_directory: &PathBuf) -> Self {
        ModelCustodian {
            directory: campaign_directory.clone(),
        }
    }
    pub fn start(&self) {}

    pub fn get_campaign_data(&self) -> Result<Vec<CampaignDto>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use tokio::sync::mpsc::channel;

    use super::*;
    #[test]
    fn test_model() {
        let model = ModelCustodian::create(&PathBuf::from(""));
        model.start();
        let actual = model.get_campaign_data().unwrap();

        assert_eq!(actual, vec![]);
    }
}
