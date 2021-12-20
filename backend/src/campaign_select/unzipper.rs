use std::{fs, io::Read, path::PathBuf};

pub struct Unzipper {}
impl Unzipper {
    pub fn get_zipped_content(zip: &PathBuf) -> (String, String) {
        let meta = Unzipper::get_file_content("meta", zip);
        let gamestate = Unzipper::get_file_content("gamestate", zip);

        (meta, gamestate)
    }

    fn get_file_content(filename: &str, filepath: &PathBuf) -> String {
        let file = fs::File::open(filepath).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();

        let mut zip_file = archive.by_name(filename).unwrap();
        let mut out = String::new();
        zip_file.read_to_string(&mut out).unwrap();
        out
    }
}
