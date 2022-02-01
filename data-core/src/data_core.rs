use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;
use sqlx::SqlitePool;

pub struct DataCore {
    _pool: SqlitePool,
}

impl DataCore {
    pub async fn create<PATH, NAME>(path: &PATH, name: &NAME) -> Result<Self>
    where
        PATH: AsRef<Path>,
        NAME: AsRef<str>,
    {
        DataCore::_create(path.as_ref(), name.as_ref()).await
    }

    async fn _create(path: &Path, name: &str) -> Result<Self> {
        let database_url = format!(
            "sqlite:{}",
            PathBuf::from_iter(vec![path.to_str().unwrap(), name])
                .to_str()
                .unwrap()
        );

        Command::new("sqlx")
            .args(&["database", "create", "--database-url", &database_url])
            .output()
            .unwrap();

        let me = DataCore {
            _pool: SqlitePool::connect(&database_url).await?,
        };

        Ok(me)
    }
}
