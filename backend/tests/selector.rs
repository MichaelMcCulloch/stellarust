use std::{fs, path::PathBuf};

fn get_resource_dir() -> PathBuf {
    let test_resource_dir = {
        let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
        dir.pop();
        dir.push("res");
        dir.push("test_data");
        dir.push("campaign");
        dir
    };
    test_resource_dir
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;
    use backend::campaign_select::selector::CampaignSelector;
    use stellarust::dto::CampaignDto;
    use time::{macros::datetime, OffsetDateTime};

    #[tokio::test]
    async fn campaign_select__get_campaign_options() {
        let test_resource_dir = {
            let mut dir = get_resource_dir();
            dir.push("unitednationsofearth_-15512622");
            dir
        };
        let expected_time = datetime!(2021-12-18 19:26:12.142231637 -7);

        let expected_dto = CampaignDto {
            name: String::from("United Nations of Earth"),
            empires: vec![
                "United Nations of Earth",
                "Yaanari Imperium",
                "Confederation of Jakaro",
                "Scyldari Confederacy",
                "Maloqui Hierarchy",
                "Desstican Monopoly",
                "Techarus Core",
                "United Panaxala Imperium",
                "Republic of Yapathinor",
                "Cormathani Trading Consortium",
                "Vivisandia Guardians",
                "Queptilium Remnant",
                "Mathin Civilization",
                "Placid Leviathans",
                "Placid Leviathans",
                "Tiyanki Space Whale Ancient",
                "Commonwealth of Man",
                "Andigonj Corsairs",
                "Curator Order",
                "Prism",
                "Artisan Troupe",
                "Caravansary Caravan Coalition",
                "The Numistic Order",
                "Racket Industrial Enterprise",
                "XuraCorp",
                "Space Amoeba Gathering",
                "Enigmatic Fortress",
                "Menjeti Freebooters",
                "Riggan Commerce Exchange",
                "Automated Dreadnought",
                "Spaceborne Organics",
                "Mineral Extraction Operation",
                "Armistice Initiative",
                "Tavurite Civilization",
                "Enigmatic Energy",
                "Xu'Lokako Civilization",
                "Sinrath Civilization",
                "Pelisimus Civilization",
                "H'Runi Civilization",
                "Belmacosa Civilization",
                "global_event_country",
                "The Shroud",
                "Creatures of the Shroud",
                "VLUUR",
            ]
            .into_iter()
            .map(|s| String::from(s))
            .collect(),
            last_write: expected_time.into(),
        };

        let map = CampaignSelector::get_campaign_options(vec![test_resource_dir.clone()]);

        let keys: Vec<CampaignDto> = map.clone().into_keys().collect();
        let actual_dto = keys.get(0).unwrap();
        let actual_path = map.get(actual_dto).unwrap();

        assert_eq!(actual_dto.name, expected_dto.name);
        assert_eq!(actual_dto.empires, expected_dto.empires);
        assert_eq!(actual_dto.last_write, expected_dto.last_write);

        assert_eq!(actual_path, &test_resource_dir);
    }
}
