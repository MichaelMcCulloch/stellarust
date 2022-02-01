pub mod utility {
    use std::{
        path::{Path, PathBuf},
        process::Command,
    };

    use anyhow::Result;

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

    pub fn create_sqlite_db<P: AsRef<Path>>(full_path: &P) -> Result<()> {
        _create_sqlite_db(full_path.as_ref())
    }
    fn _create_sqlite_db(full_path: &Path) -> Result<()> {
        let database_url = format!("sqlite:{}", full_path.to_str().unwrap());

        Command::new("sqlx")
            .args(&["database", "create", "--database-url", &database_url])
            .output()
            .unwrap();
        Ok(())
    }

    pub fn drop_sqlite_db<P: AsRef<Path>>(full_path: &P) -> Result<()> {
        _drop_sqlite_db(full_path.as_ref())
    }
    fn _drop_sqlite_db(full_path: &Path) -> Result<()> {
        let database_url = format!("sqlite:{}", full_path.to_str().unwrap());

        Command::new("sqlx")
            .args(&["database", "drop", "--database-url", &database_url, "-y"])
            .output()
            .unwrap();
        Ok(())
    }

    pub fn cleanup_sqlite<PATH, NAME>(path_root: &PATH, name: &NAME)
    where
        PATH: AsRef<Path>,
        NAME: AsRef<str>,
    {
        _cleanup_sqlite(path_root.as_ref(), name.as_ref())
    }

    fn _cleanup_sqlite(path_root: &Path, name: &str) {
        let name_shm = format!("{}-shm", name);
        let name_wal = format!("{}-wal", name);

        for file_name in vec![name, &name_shm, &name_wal].into_iter() {
            fs::remove_file(
                PathBuf::from_iter(vec![path_root.to_str().unwrap(), file_name])
                    .to_str()
                    .unwrap(),
            )
            .unwrap_or(());
        }
    }
}
