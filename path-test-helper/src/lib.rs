use std::path::PathBuf;

const CAMPAIGN_UNE_ROOT_RELATIVE: &str =
    "stellarust/res/test_data/campaign/unitednationsofearth_-15512622/";

pub fn get_test_campaign_une_root() -> PathBuf {
    get_path(CAMPAIGN_UNE_ROOT_RELATIVE)
}

pub fn get_path(path: &str) -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();
    loop {
        cwd.pop();
        if cwd.into_iter().last().unwrap() != "stellarust" {
            break;
        };
    }
    PathBuf::from_iter(vec![&cwd, &PathBuf::from(path)])
}
