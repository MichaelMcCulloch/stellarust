use super::reader::FileReader;
use std::{fs, path::PathBuf};
use stellarust::dto::CampaignDto;
use time::macros::datetime;

const SAVE_DATA_PATH: &str = ".local/share/Paradox Interactive/Stellaris/save games";

pub struct LinuxFileReader {}
impl FileReader for LinuxFileReader {
    
    fn read_from_path(path: &PathBuf) -> Vec<CampaignDto> {
        log::info!("{:?}", path);
        let paths = fs::read_dir(path).unwrap();
        let save_dtos: Vec<CampaignDto> = paths
            .filter_map(|f| {
                if let Ok(dir_entry) = f {
                    let out = CampaignDto {
                        name: format!("{}", dir_entry.path().display()),
                        empires: vec!["One".into(), "Two".into(), "Three".into()],
                        last_write: datetime!(2021-12-25 0:00 UTC),
                    };
                    Some(out)
                } else {
                    None
                }
            })
            .collect();
        save_dtos
    }

    fn read() -> Vec<CampaignDto> {
        LinuxFileReader::read_from_path(&PathBuf::from(format!(
            "{}/{}",
            std::env::var("HOME").unwrap(),
            SAVE_DATA_PATH
        )))
    }
}

#[cfg(test)]
mod tests {
    use stellarust::dto::CampaignDto;
    use time::{macros::datetime};

    use super::LinuxFileReader;
    use crate::campaign_select::reader::FileReader;
    use std::{path::PathBuf, collections::HashSet};

    const TEST_DATA_PATH: &str = ".local/share/Paradox Interactive/Stellaris/save games";

    #[tokio::test]
    async fn test_empires_returnsListOfEmpireNames() {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/{}", home, TEST_DATA_PATH);

        let expected = vec![
            CampaignDto { 
                name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/earthcustodianship2_-1731632235".into(), 
                empires: vec!["One".into(), "Two".into(), "Three".into()], 
                last_write:datetime!(2021-12-25 0:00 UTC) },
            CampaignDto {
                name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/xt489eliminator_452026845".into(), 
                empires: vec!["One".into(), "Two".into(), "Three".into()], 
                last_write:datetime!(2021-12-25 0:00 UTC) },
            CampaignDto { 
                name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/deleted_404510102".into(),
                empires: vec!["One".into(), "Two".into(), "Three".into()],
                last_write:datetime!(2021-12-25 0:00 UTC) 
            }];

        let save_dtos = LinuxFileReader::read_from_path(&PathBuf::from(path));

        let expected: HashSet<_> = expected.iter().collect();
        let actual: HashSet<_> = save_dtos.iter().collect();
        assert_eq!(actual, expected);
    }


}