#[cfg(test)]
mod data_import_tests {
    use std::{fmt::Result, fs, path::PathBuf, sync::Once};

    use backend::dirwatcher::DirectoryEventHandler;
    use data_model::ModelCustodian;
    use sqlx::SqlitePool;

    static INIT: Once = Once::new();
    const TEST_DIRECTORY_SOURCE: &str =
        "stellarust/res/test_data/campaign/unitednationsofearth_-15512622/";
    const TEST_WORKING_DIRECTORY_SOURCE: &str =
        "stellarust/res/test_data/campaign/data_import_integration_test/";

    fn get_path(path: &str) -> anyhow::Result<PathBuf> {
        let mut cwd = std::env::current_dir()?;
        loop {
            cwd.pop();
            if cwd.into_iter().last().unwrap() != "stellarust" {
                break;
            };
        }
        Ok(PathBuf::from_iter(vec![cwd.to_str().unwrap(), path]))
    }

    #[actix_rt::test]
    async fn on_data_import__no_existing_db_at_save_directory__db_is_created_and_populated() {
        let testing_data_path = get_path(TEST_DIRECTORY_SOURCE).unwrap();
        let test_working_dir_path = get_path(TEST_WORKING_DIRECTORY_SOURCE).unwrap();

        fs::create_dir_all(test_working_dir_path).unwrap();
        let (receiver, _dir_watcher) = DirectoryEventHandler::create(&testing_data_path);

        let custodian = ModelCustodian::create(receiver);
    }

    #[actix_rt::test]
    async fn on_data_import__existing_db_at_save_directory_but_unpopulated__db_is_populated() {}

    #[actix_rt::test]
    async fn on_data_import__existing_db_at_save_directory_populated_old__db_is_populated_with_new_data(
    ) {
    }

    #[actix_rt::test]
    async fn on_data_import__existing_db_at_save_directory_populated_current__db_is_unchanged() {}
}
