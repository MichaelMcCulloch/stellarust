use anyhow::Result;
use std::{fs, io::Read, path::PathBuf};
pub struct Unzipper {}
impl Unzipper {
    pub fn get_zipped_content(zip: &PathBuf) -> Result<(String, String)> {
        let meta = get_file_content("meta", zip)?;
        let gamestate = get_file_content("gamestate", zip)?;

        Ok((meta, gamestate))
    }
}
fn get_file_content(filename: &str, filepath: &PathBuf) -> Result<String> {
    let file = fs::File::open(filepath)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut zip_file = archive.by_name(filename)?;
    let mut out = String::new();
    zip_file.read_to_string(&mut out)?;
    Ok(out)
}
