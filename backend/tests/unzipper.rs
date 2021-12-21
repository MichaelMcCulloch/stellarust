use std::{fs, path::PathBuf};

fn get_resource_dir() -> PathBuf {
    let test_resource_dir = {
        let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
        dir.pop();
        dir.push("res");
        dir.push("test_data");
        dir
    };
    test_resource_dir
}

#[cfg(test)]
mod tests {
    use backend::campaign_select::unzipper::Unzipper;

    use super::*;
    #[tokio::test]
    async fn unzipper__get_zipped_content() {
        let test_resource_dir = {
            let mut dir = get_resource_dir();
            dir.push("unzipper/zipped.sav");
            dir
        };

        let (meta, gamestate) = Unzipper::get_zipped_content(&test_resource_dir).unwrap();
        assert_eq!(meta, String::from("Hello"));
        assert_eq!(gamestate, String::from("World!"));
    }
}
