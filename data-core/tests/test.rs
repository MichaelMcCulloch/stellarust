#[cfg(test)]
mod tests {
    use std::{
        fs::{self, ReadDir},
        path::PathBuf,
    };

    use data_core::DataCore;
    use path_test_helper::get_path;
    use sqlite_test_helper::{cleanup_sqlite, create_sqlite_db};

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

        let directory_file_names: Vec<_> = fs::read_dir(&test_working_dir_path)
            .unwrap()
            .filter_map(|r| {
                if let Ok(x) = r {
                    Some(x.file_name().into_string().unwrap())
                } else {
                    None
                }
            })
            .collect();

        cleanup_sqlite(&test_working_dir_path, &NO_DB);

        assert!(directory_file_names.contains(&NO_DB.to_string()));
    }

    #[actix_rt::test]
    async fn create__given_path_db_at_path__sqlite_db_unchanged_at_path() {
        let test_working_dir_path = get_path(TEST_WORKING_DIRECTORY_SOURCE);

        fs::create_dir_all(test_working_dir_path.clone()).unwrap();
        cleanup_sqlite(&test_working_dir_path, &EXISTING_DB);

        create_sqlite_db(&test_working_dir_path, &EXISTING_DB).unwrap();

        let last_modified_original = fs::metadata(&PathBuf::from_iter(vec![
            &test_working_dir_path,
            &PathBuf::from(EXISTING_DB),
        ]))
        .unwrap()
        .modified()
        .unwrap();

        let core = DataCore::create(&test_working_dir_path, &EXISTING_DB)
            .await
            .unwrap();

        let last_modified_after_create = fs::metadata(&PathBuf::from_iter(vec![
            &test_working_dir_path,
            &PathBuf::from(EXISTING_DB),
        ]))
        .unwrap()
        .modified()
        .unwrap();

        cleanup_sqlite(&test_working_dir_path, &EXISTING_DB);

        assert_eq!(last_modified_original, last_modified_after_create);
        //TODO assert contents unchanged
    }
}
