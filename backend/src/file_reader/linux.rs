use super::reader::FileReader;
use std::{fs, path::PathBuf};
use stellarust::dto::SaveGameDto;
use time::macros::datetime;

const SAVE_DATA_PATH: &str = "$HOME/.local/share/Paradox Interactive/Stellaris/save games";

pub struct LinuxFileReader {}

impl FileReader for LinuxFileReader {
    fn read_from_path(path: &PathBuf) -> Vec<SaveGameDto> {
        let paths = fs::read_dir(path).unwrap();
        let save_dtos: Vec<SaveGameDto> = paths
            .filter_map(|f| {
                if let Ok(dir_entry) = f {
                    let out = SaveGameDto {
                        save_name: format!("{}", dir_entry.path().display()),
                        empires: vec![],
                        last_save_zoned_date_time: datetime!(2021-12-25 0:00 UTC),
                    };
                    Some(out)
                } else {
                    None
                }
            })
            .collect();
        save_dtos
    }

    fn read() -> Vec<SaveGameDto> {
        LinuxFileReader::read_from_path(&PathBuf::from(format!(
            "{}/{}",
            std::env::var("HOME").unwrap(),
            SAVE_DATA_PATH
        )))
    }
}

#[cfg(test)]
mod tests {
    use stellarust::dto::SaveGameDto;
    use time::{macros::datetime, Date, OffsetDateTime, PrimitiveDateTime};

    use super::LinuxFileReader;
    use crate::file_reader::reader::FileReader;
    use std::{fs, path::PathBuf, collections::HashSet};

    const TEST_DATA_PATH: &str = ".local/share/Paradox Interactive/Stellaris/save games";

    #[tokio::test]
    async fn linux() {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/{}", home, TEST_DATA_PATH);
        let paths = fs::read_dir(path).unwrap();
        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
    }

    #[tokio::test]
    async fn test_empires_returnsListOfEmpireNames() {
        let home = std::env::var("HOME").unwrap();
        let path = format!("{}/{}", home, TEST_DATA_PATH);

        let expected = vec![
            SaveGameDto { 
                save_name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/earthcustodianship2_-1731632235".into(), 
                empires: vec![], 
                last_save_zoned_date_time:datetime!(2021-12-25 0:00 UTC) },
            SaveGameDto {
                save_name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/xt489eliminator_452026845".into(), 
                empires: vec![], 
                last_save_zoned_date_time:datetime!(2021-12-25 0:00 UTC) },
            SaveGameDto { 
                save_name: "/home/michael/.local/share/Paradox Interactive/Stellaris/save games/deleted_404510102".into(),
                empires: vec![],
                last_save_zoned_date_time:datetime!(2021-12-25 0:00 UTC) 
            }];

        let save_dtos = LinuxFileReader::read_from_path(&PathBuf::from(path));

        let expected: HashSet<_> = expected.iter().collect();
        let actual: HashSet<_> = save_dtos.iter().collect();
        assert_eq!(actual, expected);
    }


}
