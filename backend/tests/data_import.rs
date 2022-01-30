#[cfg(test)]
mod data_import_tests {
    use std::{
        any, fmt::Result, fs, path::PathBuf, process::Command, sync::Once, thread, time::Duration,
    };

    use backend::dirwatcher::DirectoryEventHandler;
    use data_model::ModelCustodian;
    use sqlx::SqlitePool;

    fn TEST_DIRECTORY_SOURCE() -> PathBuf {
        PathBuf::from("stellarust/res/test_data/campaign/unitednationsofearth_-15512622/")
    }
    fn TEST_WORKING_DIRECTORY_SOURCE() -> PathBuf {
        PathBuf::from("stellarust/res/test_data/data_import/")
    }

    fn get_path(path: &PathBuf) -> anyhow::Result<PathBuf> {
        let mut cwd = std::env::current_dir()?;
        loop {
            cwd.pop();
            if cwd.into_iter().last().unwrap() != "stellarust" {
                break;
            };
        }
        Ok(PathBuf::from_iter(vec![&cwd, path]))
    }

    fn create_sqlite_db(full_path: &PathBuf) -> anyhow::Result<()> {
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

    fn drop_sqlite_db(full_path: &PathBuf) -> anyhow::Result<()> {
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

    #[actix_rt::test]
    async fn on_data_import__fresh_database_and_new_data__db_is_populated() {
        let testing_data_path = get_path(&TEST_DIRECTORY_SOURCE()).unwrap();
        let test_working_dir_path = get_path(&TEST_WORKING_DIRECTORY_SOURCE()).unwrap();

        fs::create_dir_all(test_working_dir_path.clone()).unwrap();

        let sqlite_db_path =
            PathBuf::from_iter(vec![test_working_dir_path, PathBuf::from("fresh_db")]);

        let (receiver, _dir_watcher) = DirectoryEventHandler::create(&testing_data_path);

        let custodian = ModelCustodian::create(receiver);
    }

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_same_data__db_is_untouched() {
        let testing_data_path = get_path(&TEST_DIRECTORY_SOURCE()).unwrap();
        let test_working_dir_path = get_path(&TEST_WORKING_DIRECTORY_SOURCE()).unwrap();

        let sqlite_db_path =
            PathBuf::from_iter(vec![test_working_dir_path, PathBuf::from("existing_db")]);
        create_sqlite_db(&sqlite_db_path).unwrap();

        let poolA = SqlitePool::connect(&sqlite_db_path.to_str().unwrap())
            .await
            .unwrap();

        thread::sleep(Duration::from_secs(1));
        drop_sqlite_db(&sqlite_db_path).unwrap();
    }

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_new_data__db_is_populated_with_new_data() {}
}
