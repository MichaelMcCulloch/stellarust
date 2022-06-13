#[cfg(test)]
mod data_import_tests {

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
