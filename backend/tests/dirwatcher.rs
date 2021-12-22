use std::path::PathBuf;

#[cfg(test)]
mod tests {

    use super::*;
    use backend::{
        dirwatcher::DirectoryEventHandler,
        model::{CustodianMsg, ModelDataPoint},
    };
    use std::{fs, io::Write, thread, time::Duration};

    #[test]
    fn test_dir_watcher__receiver__new_files_are_in_receiver_queue() {
        let dir = get_resource_dir();

        let existing_files = get_test_file_paths(0);
        for (index, path) in existing_files.clone().into_iter().enumerate() {
            fs::write(path, format!("{}", index)).unwrap();
        }

        let (msg_receiver, _watcher) = DirectoryEventHandler::create(&dir);

        msg_receiver.recv_timeout(Duration::from_secs(2)).unwrap();
        msg_receiver.recv_timeout(Duration::from_secs(2)).unwrap();
        msg_receiver.recv_timeout(Duration::from_secs(2)).unwrap();

        let new_files = get_test_file_paths(3);
        for (index, path) in new_files.clone().into_iter().enumerate() {
            fs::write(path, format!("{}", index)).unwrap();
        }

        let result = msg_receiver.recv_timeout(Duration::from_secs(2));
        for path in vec![existing_files, new_files].concat() {
            fs::remove_file(path).unwrap();
        }
        match result {
            Ok(msg) => assert_eq!(CustodianMsg::Data(ModelDataPoint { data: 0 }), msg),
            Err(error) => panic!("{}", error),
        }
    }

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

    fn get_test_file_paths(start_at: usize) -> Vec<PathBuf> {
        let file_paths: Vec<PathBuf> = vec![
            format!("f{}", start_at),
            format!("f{}", start_at + 1),
            format!("f{}", start_at + 2),
        ]
        .into_iter()
        .map(|file_name| {
            let mut res = get_resource_dir();
            res.push(file_name);
            res
        })
        .collect();
        file_paths
    }
}
