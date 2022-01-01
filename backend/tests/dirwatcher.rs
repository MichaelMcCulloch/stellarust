use std::path::PathBuf;

#[cfg(test)]
mod tests {

    use super::*;
    use backend::dirwatcher::DirectoryEventHandler;
    use model::{CustodianMsg, ModelDataPoint};
    use std::{fs, time::Duration};

    #[test]
    fn test_dir_watcher__receiver__existing_and_new_files_are_in_receiver_queue() {
        let source_files = get_test_source_files();
        let test_files = get_test_files();

        for i in 0..2 {
            fs::copy(&source_files[i], &test_files[i]).unwrap();
        }

        let (msg_receiver, _watcher) = DirectoryEventHandler::create(&get_test_dir());

        msg_receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        msg_receiver.recv_timeout(Duration::from_secs(10)).unwrap();

        for i in 2..4 {
            fs::copy(&source_files[i], &test_files[i]).unwrap();
        }

        let result = msg_receiver.recv_timeout(Duration::from_secs(10));
        for path in test_files {
            fs::remove_file(path).unwrap();
        }
        match result {
            Ok(msg) => assert_eq!(CustodianMsg::Data(ModelDataPoint { data: 0 }), msg),
            Err(error) => panic!("{}", error),
        }
    }

    fn get_test_source_files() -> Vec<PathBuf> {
        get_paths_from_root(get_file_names(), &get_test_source_dir())
    }

    fn get_test_files() -> Vec<PathBuf> {
        get_paths_from_root(get_file_names(), &get_test_dir())
    }

    fn get_test_source_dir() -> PathBuf {
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

    fn get_test_dir() -> PathBuf {
        let test_resource_dir = {
            let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
            dir.pop();
            dir.push("res");
            dir.push("test_data");
            dir.push("dirwatcher");
            dir
        };
        test_resource_dir
    }

    fn get_paths_from_root(file_names: Vec<&str>, root: &PathBuf) -> Vec<PathBuf> {
        file_names
            .into_iter()
            .map(|file_name| {
                let mut res = root.clone();
                res.push(file_name);
                res
            })
            .collect()
    }

    fn get_file_names() -> Vec<&'static str> {
        vec![
            "autosave_2200.02.01.sav",
            "autosave_2200.03.01.sav",
            "autosave_2200.04.01.sav",
            "autosave_2200.05.01.sav",
        ]
    }
}
