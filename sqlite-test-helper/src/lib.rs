use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;

pub fn create_sqlite_db<PATH, NAME>(path_root: &PATH, name: &NAME) -> Result<()>
where
    PATH: AsRef<Path>,
    NAME: AsRef<str>,
{
    _create_sqlite_db(path_root.as_ref(), name.as_ref())
}

fn _create_sqlite_db(path_root: &Path, name: &str) -> Result<()> {
    let database_url = format!(
        "sqlite:{}",
        PathBuf::from_iter(vec![path_root.to_str().unwrap(), name])
            .to_str()
            .unwrap()
    );

    Command::new("sqlx")
        .args(&["database", "create", "--database-url", &database_url])
        .output()
        .unwrap();
    Ok(())
}

pub fn drop_sqlite_db<PATH, NAME>(path_root: &PATH, name: &NAME) -> Result<()>
where
    PATH: AsRef<Path>,
    NAME: AsRef<str>,
{
    _drop_sqlite_db(path_root.as_ref(), name.as_ref())
}
fn _drop_sqlite_db(path_root: &Path, name: &str) -> Result<()> {
    let database_url = format!(
        "sqlite:{}",
        PathBuf::from_iter(vec![path_root.to_str().unwrap(), name])
            .to_str()
            .unwrap()
    );

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
