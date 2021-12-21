use std::path::PathBuf;

fn get_resource_dir() -> PathBuf {
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

fn get_test_file_paths() -> Vec<PathBuf> {
    let file_paths: Vec<PathBuf> = vec!["f1, f2, f3"]
        .into_iter()
        .map(|file_name| {
            let mut res = get_resource_dir();
            res.push(file_name);
            res
        })
        .collect();
    file_paths
}

#[cfg(test)]
mod tests {

    use super::*;
    use backend::{
        dirwatcher::DirectoryEventHandler,
        model::{CustodianMsg, ModelDataPoint},
    };
    use std::{fs, io::Write, thread, time::Duration};

    #[test]
    fn test_dir_watcher__oncreate__existing_files_are_in_receiver_queue() {
        let _new_files = vec!["f1, f2, f3"];
    }

    #[test]
    fn test_dir_watcher__receiver__new_files_are_in_receiver_queue() {
        let file_paths = get_test_file_paths();

        let dir = get_resource_dir();

        let (msg_receiver, _watcher) = DirectoryEventHandler::create(&dir);

        for (index, path) in file_paths.clone().into_iter().enumerate() {
            fs::write(path, format!("{}", index)).unwrap();
        }

        let result = msg_receiver.recv_timeout(Duration::from_secs(10));
        for path in file_paths {
            fs::remove_file(path).unwrap();
        }
        match result {
            Ok(msg) => assert_eq!(CustodianMsg::Data(ModelDataPoint { data: 0 }), msg),
            Err(error) => panic!("{}", error),
        }
    }
}
