use crate::{
    parser::{DataImport, DataImportResult},
    unzipper::Unzipper,
};
use anyhow::Result;
use data_model::ModelDataPoint;
use std::{collections::HashMap, fs, path::PathBuf, time::SystemTime};
use stellarust::dto::CampaignDto;

pub fn get_campaign_options(paths: Vec<PathBuf>) -> Result<HashMap<CampaignDto, PathBuf>> {
    let mut map: HashMap<CampaignDto, PathBuf> = HashMap::new();
    for path in paths {
        map.insert(get_campaign_option(&path)?, path);
    }
    Ok(map)
}

fn get_campaign_option(path: &PathBuf) -> Result<CampaignDto> {
    let paths = std::fs::read_dir(path)?;
    let (modified, most_recent_path) = find_newest_save(paths)?;

    let (meta_str, gamestate_str) = Unzipper::get_zipped_content(&most_recent_path)?;

    let meta = DataImport::from_meta(meta_str.as_str())?;
    let gamestate = DataImport::from_gamestate(gamestate_str.as_str())?;

    let model = ModelDataPoint::from(DataImportResult { meta, gamestate });

    Ok(CampaignDto {
        name: model.campaign_name,
        empires: model
            .empires
            .into_iter()
            .map(|empire| empire.name)
            .collect(),
        last_write: modified,
    })
}

fn find_newest_save(paths: fs::ReadDir) -> Result<(SystemTime, PathBuf)> {
    let (modified, most_recent_path) = paths
        .map(|file_result| {
            let save_file = file_result?;
            let metadata = save_file.metadata()?;
            Ok((metadata.modified()?, save_file.path()))
        })
        .filter_map(|result: Result<(SystemTime, PathBuf)>| {
            if let Ok(name) = result {
                Some(name)
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
    Ok((modified, most_recent_path))
}
