use std::{fs, path::PathBuf};

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

#[cfg(test)]
mod tests {
    use super::*;
    use backend::campaign_select::selector::CampaignSelector;
    use stellarust::dto::CampaignDto;
    use time::macros::datetime;

    #[tokio::test]
    async fn campaign_select__get_campaign_option() {
        let test_resource_dir = {
            let mut dir = get_resourc_dir();
            dir.push("unitednationsofearth_-15512622");
            dir
        };

        let expected_dto = CampaignDto {
            name: String::from("United Nations of Earth"),
            empires: vec![],
            last_write: datetime!(2021-12-19 19:26 -7),
        };

        let actual_dto = CampaignSelector::get_campaign_option(&test_resource_dir);

        assert_eq!(actual_dto.name, expected_dto.name);
        assert_eq!(actual_dto.empires, expected_dto.empires);
        assert_eq!(actual_dto.last_write, expected_dto.last_write);
    }
}
