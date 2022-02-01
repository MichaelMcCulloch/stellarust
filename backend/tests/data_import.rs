#[cfg(test)]
mod data_import_tests {
    use std::{fs, path::PathBuf, thread, time::Duration};

    use backend::dirwatcher::DirectoryEventHandler;
    use data_core::DataCore;
    use sqlx::SqlitePool;
    use test_helper::{cleanup_sqlite, create_sqlite_db, get_path, get_test_campaign_une_root};

    const TEST_WORKING_DIRECTORY_SOURCE: &str = "stellarust/res/test_data/data_import/";
    const FRESH_DB: &str = "FRESH_DB";
    const EXISTING_DB: &str = "EXISTING_DB";

    #[actix_rt::test]
    async fn on_data_import__fresh_database_and_new_data__db_is_populated() {
        let testing_data_path = &get_test_campaign_une_root();
        let test_working_dir_path = &get_path(TEST_WORKING_DIRECTORY_SOURCE);

        fs::create_dir_all(test_working_dir_path.clone()).unwrap();

        let (receiver, _dir_watcher) = DirectoryEventHandler::create(&testing_data_path);
        // TODO: Factor out to a database custodian

        let db_location = PathBuf::from_iter(vec![test_working_dir_path, &PathBuf::from(FRESH_DB)]);
        let core = DataCore::create(&test_working_dir_path, &FRESH_DB)
            .await
            .unwrap();

        let pool = SqlitePool::connect(&db_location.to_str().unwrap());

        cleanup_sqlite(&test_working_dir_path, &FRESH_DB);
    }

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_same_data__db_is_untouched() {
        let testing_data_path = &get_test_campaign_une_root();
        let test_working_dir_path = &get_path(TEST_WORKING_DIRECTORY_SOURCE);

        let db_location =
            PathBuf::from_iter(vec![test_working_dir_path, &PathBuf::from(EXISTING_DB)]);
        create_sqlite_db(&test_working_dir_path, &EXISTING_DB).unwrap();

        let poolA = SqlitePool::connect(&db_location.to_str().unwrap())
            .await
            .unwrap();

        thread::sleep(Duration::from_secs(1));
        cleanup_sqlite(&test_working_dir_path, &EXISTING_DB);
    }

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_new_data__db_is_populated_with_new_data() {}
}
