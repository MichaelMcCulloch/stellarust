use crate::campaign_select::unzipper::Unzipper;
use std::{collections::HashMap, fs, path::PathBuf, time::SystemTime};
use stellarust::dto::CampaignDto;

pub fn get_campaign_options(paths: Vec<PathBuf>) -> HashMap<CampaignDto, PathBuf> {
    let mut map: HashMap<CampaignDto, PathBuf> = HashMap::new();
    for path in paths {
        map.insert(get_campaign_option(&path), path);
    }
    map
}

fn get_campaign_option(path: &PathBuf) -> CampaignDto {
    let paths = std::fs::read_dir(path).unwrap();
    let (modified, most_recent_path) = find_newest_save(paths);

    let (meta, gamestate) = Unzipper::get_zipped_content(&most_recent_path);

    CampaignDto {
        name: get_name_from_meta(meta),
        empires: get_empires_from_gamestate(gamestate),
        last_write: modified,
    }
}

fn find_newest_save(paths: fs::ReadDir) -> (SystemTime, PathBuf) {
    let (modified, most_recent_path) = paths
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
    (modified, most_recent_path)
}

fn get_name_from_meta(meta: String) -> String {
    let lines = meta.split('\n');
    let name_line_vec = lines
        .into_iter()
        .filter(|l| l.starts_with("name="))
        .collect::<Vec<&str>>();
    let name_line = name_line_vec.get(0).unwrap();
    let name_assignment_vec = name_line.split("=").collect::<Vec<&str>>();
    let name = name_assignment_vec.get(1).unwrap();
    let parsed_name: String = serde_json::from_str(name).unwrap();
    parsed_name
}

fn get_empires_from_gamestate(gamestate: String) -> Vec<String> {
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
            let name = name_assignment_vec.get(1).unwrap();
            let parsed_name: String = serde_json::from_str(name).unwrap();
            String::from(parsed_name)
        })
        .collect();
    names
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_name_from_meta__truncated__happy_path__returns_empire_name() {
        let meta_string = "name=\"United Nations of Whatever\"\n";
        let name = get_name_from_meta(meta_string.into());
        assert_eq!(name, "United Nations of Whatever");
    }

    #[test]
    fn get_empires_from_gamestate__truncated__happy_path__returns_empire_name() {
        let gamestate_string =
            "color_index=-1\nwhatever_you_like_here=\"United Nations of Whatever\"\n";
        let empires = get_empires_from_gamestate(gamestate_string.into());
        assert!(empires.contains(&String::from("United Nations of Whatever")));
    }
}
