use std::path::PathBuf;

use stellarust::dto::SaveGameDto;

pub(crate) trait FileReader {
    fn read_from_path(path: &PathBuf) -> Vec<SaveGameDto>;
    fn read() -> Vec<SaveGameDto>;
}
