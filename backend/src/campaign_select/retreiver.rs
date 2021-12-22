use crate::unzipper::Unzipper;
use anyhow::Result;
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

    let (meta, gamestate) = Unzipper::get_zipped_content(&most_recent_path)?;

    Ok(CampaignDto {
        name: get_name_from_meta(meta)?,
        empires: get_empires_from_gamestate(gamestate)?,
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

fn get_name_from_meta(meta: String) -> Result<String> {
    let lines = meta.split('\n');
    let indicator = "name=";
    let name_line_vec = lines
        .into_iter()
        .filter(|l| l.starts_with(indicator))
        .collect::<Vec<&str>>();
    let name_line = name_line_vec
        .get(0)
        .expect(format!("No found beginning with '{}'.", indicator).as_str());
    let name_assignment_vec = name_line.split("=").collect::<Vec<&str>>();
    let name = name_assignment_vec
        .get(1)
        .expect("Name not assigned to anything.");
    let parsed_name: String = serde_json::from_str(name)?;
    Ok(parsed_name)
}

fn get_empires_from_gamestate(gamestate: String) -> Result<Vec<String>> {
    let indicator = "color_index";
    let indicated_line_numbers: Vec<usize> = gamestate
        .split('\n')
        .enumerate()
        .filter(|(_, line)| line.contains(indicator))
        .map(|(index, _)| index + 1)
        .collect();

    let names: Vec<String> = gamestate
        .split('\n')
        .enumerate()
        .filter(|(index, _)| indicated_line_numbers.contains(index))
        .map(|(_, line)| {
            let name_assignment_vec = line.split("=").collect::<Vec<&str>>();
            let name = name_assignment_vec.get(1).expect("");
            let parsed_name: String = serde_json::from_str(name)?;
            Ok(String::from(parsed_name))
        })
        .filter_map(|result: Result<String>| {
            if let Ok(name) = result {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    Ok(names)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_name_from_meta__truncated__happy_path__returns_empire_name() {
        let meta_string = "name=\"United Nations of Whatever\"\n";
        let name = get_name_from_meta(meta_string.into()).unwrap();
        assert_eq!(name, "United Nations of Whatever");
    }

    #[test]
    fn get_empires_from_gamestate__truncated__happy_path__returns_empire_name() {
        let gamestate_string =
            "color_index=-1\nwhatever_you_like_here=\"United Nations of Whatever\"\n";
        let empires = get_empires_from_gamestate(gamestate_string.into()).unwrap();
        assert!(empires.contains(&String::from("United Nations of Whatever")));
    }
}
