#[cfg(test)]
mod tests {
    use super::*;
    use backend::campaign_select::selector::CampaignSelector;
    use stellarust::dto::CampaignDto;
    use time::macros::datetime;

    use std::{fs, path::PathBuf};
    #[tokio::test]
    async fn campaign_select__get_campaign_option() {
        let test_resource_dir = {
            let mut dir = get_resourc_dir();
            dir.push("unitednationsofearth_-15512622");
            dir
        };

        let actual_dto = CampaignSelector::get_campaign_option(test_resource_dir);

        assert_eq!(
            actual_dto,
            CampaignDto {
                name: "United Nations of Earth".into(),
                empires: vec![],
                last_write: datetime!(2021-12-19 19:26 -7)
            }
        )
    }

    fn get_resourc_dir() -> PathBuf {
        let test_resource_dir = {
            let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
            dir.pop();
            dir.push("res");
            dir.push("test_data");
            dir
        };
        test_resource_dir
    }

    use std::{fs, path::PathBuf};
    #[tokio::test]
    async fn campaign_select__get_zipped_content() {
        let test_resource_dir = {
            let mut dir = get_resourc_dir();
            dir.push("unitednationsofearth_-15512622/autosave_2200.05.01.sav");
            dir
        };

        let actual_dto = CampaignSelector::get_zipped_content(test_resource_dir);

        assert_eq!(
            actual_dto,
            CampaignDto {
                name: "United Nations of Earth".into(),
                empires: vec![],
                last_write: datetime!(2021-12-19 19:26 -7)
            }
        )
    }

    fn get_resourc_dir() -> PathBuf {
        let test_resource_dir = {
            let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
            dir.pop();
            dir.push("res");
            dir.push("test_data");
            dir
        };
        test_resource_dir
    }
}
