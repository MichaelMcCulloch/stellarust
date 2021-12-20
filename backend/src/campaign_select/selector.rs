use super::reader::FileReader;
use crate::campaign_select::{unzipper::Unzipper, SaveFileReader};
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

    pub fn get_campaign_option(path: &PathBuf) -> CampaignDto {
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

        let (meta, gamestate) = Unzipper::get_zipped_content(&most_recent_path);

        CampaignDto {
            name: get_name_from_meta(meta),
            empires: vec![],
            last_write: OffsetDateTime::from(modified),
        }
    }
}

fn get_name_from_meta(meta: String) -> String {
    let lines = meta.split('\n');
    let name_lines = lines
        .into_iter()
        .filter(|l| l.starts_with("name="))
        .collect::<Vec<&str>>();
    let name_line = name_lines.get(0).unwrap();
    let name = name_line.split("=").collect::<Vec<&str>>();
    let namemn = *name.get(1).unwrap();
    let s: String = serde_json::from_str(namemn).unwrap();
    s
}
