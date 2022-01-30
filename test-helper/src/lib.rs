pub mod utility {
    use std::{path::PathBuf, process::Command};

    use anyhow::Result;

    const CAMPAIGN_UNE_ROOT_RELATIVE: &str =
        "stellarust/res/test_data/campaign/unitednationsofearth_-15512622/";

    pub fn get_test_campaign_une_root() -> PathBuf {
        get_path(CAMPAIGN_UNE_ROOT_RELATIVE).unwrap()
    }

    pub fn get_path(path: &str) -> Result<PathBuf> {
        let mut cwd = std::env::current_dir()?;
        loop {
            cwd.pop();
            if cwd.into_iter().last().unwrap() != "stellarust" {
                break;
            };
        }
        Ok(PathBuf::from_iter(vec![&cwd, &PathBuf::from(path)]))
    }

    pub fn create_sqlite_db(full_path: &PathBuf) -> Result<()> {
        let database_url = format!("sqlite:{}", full_path.to_str().unwrap());

        Command::new("sqlx")
            .args(&[
                "database",
                "create",
                "--database-url",
                database_url.as_str(),
            ])
            .output()
            .unwrap();
        Ok(())
    }

    pub fn drop_sqlite_db(full_path: &PathBuf) -> Result<()> {
        let database_url = format!("sqlite:{}", full_path.to_str().unwrap());

        Command::new("sqlx")
            .args(&[
                "database",
                "drop",
                "--database-url",
                database_url.as_str(),
                "-y",
            ])
            .output()
            .unwrap();
        Ok(())
    }
}
