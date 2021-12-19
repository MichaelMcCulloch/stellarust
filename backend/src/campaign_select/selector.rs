use super::reader::FileReader;
use crate::campaign_select::SaveFileReader;
use std::{arch, borrow::Borrow, collections::HashMap, fs, path::PathBuf};
use stellarust::dto::CampaignDto;
use time::OffsetDateTime;

#[cfg(target_os = "linux")]
const SAVE_DATA_PATH: &str = ".local/share/Paradox Interactive/Stellaris/save games";

pub struct CampaignSelector {}

impl CampaignSelector {
    pub fn select() -> PathBuf {
        Self::select_from_path(&PathBuf::from(SAVE_DATA_PATH))
    }

    pub fn select_from_path(dir: &PathBuf) -> PathBuf {
        let campaign_options = SaveFileReader::read();

        PathBuf::new()
    }

    fn get_campaign_options(paths: Vec<PathBuf>) -> HashMap<CampaignDto, PathBuf> {
        HashMap::new()
    }

    fn get_campaign_option(path: PathBuf) -> CampaignDto {
        println!("{}", path.display());
        let paths = std::fs::read_dir(path).unwrap();
        let (modified, most_recent_path) = paths
            .into_iter()
            .filter_map(|file_result| {
                if let Ok(file) = file_result {
                    let metadata = file.metadata().unwrap();
                    Some((metadata.modified().unwrap(), file.path()))
                } else {
                    None
                }
            })
            .reduce(|most_recent, item| {
                let (item_access, _) = item;
                let (most_recent_access, _) = most_recent;
                if item_access > most_recent_access {
                    item
                } else {
                    most_recent
                }
            })
            .unwrap();

        let content = Self::get_zipped_content(&most_recent_path);
        //get latest modified file and modify date
        //name from zipped metadata
        //empires from zipped game.countries

        CampaignDto {
            name: format!("{}", 0),
            empires: vec![],
            last_write: OffsetDateTime::from(modified),
        }
    }

    fn get_zipped_content(zip: &PathBuf) -> (String, String) {
        let file = fs::File::open(zip).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let mut v: Vec<String> = vec![];

        let meta = archive.by_name("meta").unwrap();
        let meta = archive.by_name("gamestate").unwrap();

        (String::new(), String::new())
    }
}
