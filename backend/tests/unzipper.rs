#[cfg(test)]
mod unzipper_tests {
    use backend::unzipper::Unzipper;
    use test_helper::get_path;
    const TEST_ZIP_FILE: &str = "stellarust/res/test_data/unzipper/zipped.sav";
    #[test]
    fn unzipper__get_zipped_content() {
        let test_resource_dir = get_path(TEST_ZIP_FILE);

        let (meta, gamestate) = Unzipper::get_zipped_content(&test_resource_dir).unwrap();

        assert_eq!(meta, String::from("Hello"));
        assert_eq!(gamestate, String::from("World!"));
    }
}
