#[cfg(test)]
mod data_import_tests {
    use std::{fs, path::PathBuf, thread, time::Duration};

    use backend::dirwatcher::DirectoryEventHandler;
    use data_model::ModelCustodian;
    use sqlx::SqlitePool;
    use test_helper::utility::{
        create_sqlite_db, drop_sqlite_db, get_path, get_test_campaign_une_root,
    };

    const TEST_WORKING_DIRECTORY_SOURCE: &str = "stellarust/res/test_data/data_import/";

    #[actix_rt::test]
    async fn on_data_import__fresh_database_and_new_data__db_is_populated() {
        let testing_data_path = get_test_campaign_une_root();
        let test_working_dir_path = get_path(TEST_WORKING_DIRECTORY_SOURCE).unwrap();

        fs::create_dir_all(test_working_dir_path.clone()).unwrap();

        let sqlite_db_path =
            PathBuf::from_iter(vec![test_working_dir_path, PathBuf::from("fresh_db")]);

        let (receiver, _dir_watcher) = DirectoryEventHandler::create(&testing_data_path);
        // TODO: Factor out to a database custodian
        create_sqlite_db(&sqlite_db_path);
        let pool = SqlitePool::connect(&sqlite_db_path.to_str().unwrap());

        let custodian = ModelCustodian::create(receiver);
    }

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_same_data__db_is_untouched() {
        let testing_data_path = get_test_campaign_une_root();
        let test_working_dir_path = get_path(TEST_WORKING_DIRECTORY_SOURCE).unwrap();

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
