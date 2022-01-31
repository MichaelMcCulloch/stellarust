use std::{
    path::{Path, PathBuf},
    sync::mpsc::{channel, Receiver},
};

use notify::{raw_watcher, INotifyWatcher, RawEvent, Watcher};

use super::DirWatcher;

pub struct LinuxWatcher {
    watcher: INotifyWatcher,
}

impl DirWatcher for LinuxWatcher {
    fn create(path: &Path) -> (Receiver<RawEvent>, Self) {
        let (sender, receiver) = channel();
        let mut watcher = raw_watcher(sender).unwrap();
        watcher
            .watch(path, notify::RecursiveMode::NonRecursive)
            .unwrap();

        (receiver, Self { watcher })
    }
}
