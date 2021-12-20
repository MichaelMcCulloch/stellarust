use std::path::PathBuf;

fn get_resource_dir() -> PathBuf {
    let test_resource_dir = {
        let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
        dir.pop();
        dir.push("res");
        dir.push("test_data");
        dir.push("campaign");
        dir.push("unitednationsofearth_-15512622");
        dir
    };
    test_resource_dir
}

#[cfg(test)]
mod tests {
    use backend::model::ModelCustodian;
    use std::path::PathBuf;
    use tokio::sync::mpsc::channel;

    use super::*;
    #[test]
    fn test_model() {
        let model = ModelCustodian::create(&get_resource_dir());
        model.start();
        let actual = model.get_campaign_data().unwrap();

        assert_eq!(actual, vec![]);
    }
}
