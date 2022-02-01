#[cfg(test)]
mod tests {
    use std::{
        fs::{self, ReadDir},
        path::PathBuf,
    };

    use data_core::DataCore;
    use test_helper::utility::{cleanup_sqlite, create_sqlite_db, drop_sqlite_db, get_path};

    const TEST_WORKING_DIRECTORY_SOURCE: &str = "stellarust/res/test_data/data_core/";
    const NO_DB: &str = "NO_DB.db";
    const EXISTING_DB: &str = "EXISTING_DB.db";

    #[actix_rt::test]
    async fn create__given_path_no_db_at_path__sqlite_db_created_at_path() {
        let test_working_dir_path = get_path(TEST_WORKING_DIRECTORY_SOURCE);

        fs::create_dir_all(test_working_dir_path.clone()).unwrap();

        cleanup_sqlite(&test_working_dir_path, &NO_DB);

        let core = DataCore::create(&test_working_dir_path, &NO_DB)
            .await
            .unwrap();

        let x: Vec<_> = fs::read_dir(&test_working_dir_path)
            .unwrap()
            .filter_map(|r| if let Ok(x) = r { Some(x) } else { None })
            .filter(|entry| entry.file_name() == NO_DB)
            .collect();

        cleanup_sqlite(&test_working_dir_path, &NO_DB);

        assert_eq!(x.len(), 1);
    }

}
