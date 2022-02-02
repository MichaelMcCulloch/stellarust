#[cfg(test)]
mod data_import_tests {
    use backend::dirwatcher::DirectoryEventHandler;
    use data_core::DataCore;
    use path_test_helper::{get_path, get_test_campaign_une_root};
    use sqlite_test_helper::{cleanup_sqlite, create_sqlite_db};
    use sqlx::SqlitePool;
    use std::{fs, path::PathBuf, thread, time::Duration};

    const TEST_WORKING_DIRECTORY_SOURCE: &str = "stellarust/res/test_data/data_import/";
    const FRESH_DB: &str = "FRESH_DB";
    const EXISTING_DB: &str = "EXISTING_DB";

    #[actix_rt::test]
    async fn on_data_import__fresh_database_and_new_data__db_is_populated() {}

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_same_data__db_is_untouched() {}

    #[actix_rt::test]
    async fn on_data_import__existing_database_and_new_data__db_is_populated_with_new_data() {}
}
