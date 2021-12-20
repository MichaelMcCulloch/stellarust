use super::reader::FileReader;
use std::{fs, path::PathBuf};
use stellarust::dto::CampaignDto;
use time::macros::datetime;

const SAVE_DATA_PATH: &str = ".local/share/Paradox Interactive/Stellaris/save games";

pub struct LinuxFileReader {}
impl FileReader for LinuxFileReader {
    fn read_from_path(path: &PathBuf) -> Vec<CampaignDto> {
        // log::info!("{:?}", path);
        // let paths = fs::read_dir(path).unwrap();
        // let save_dtos: Vec<CampaignDto> = paths
        //     .filter_map(|f| {
        //         if let Ok(dir_entry) = f {
        //             let out = CampaignDto {
        //                 name: format!("{}", dir_entry.path().display()),
        //                 empires: vec!["One".into(), "Two".into(), "Three".into()],
        //                 last_write: datetime!(2021-12-25 0:00 UTC),
        //             };
        //             Some(out)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        // save_dtos
        todo!()
    }

    fn read() -> Vec<CampaignDto> {
        // LinuxFileReader::read_from_path(&PathBuf::from(format!(
        //     "{}/{}",
        //     std::env::var("HOME").unwrap(),
        //     SAVE_DATA_PATH
        // )))

        todo!()
    }
}

#[cfg(test)]
mod tests {}
