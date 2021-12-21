use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
};

use anyhow::Result;
use notify::{raw_watcher, INotifyWatcher, RawEvent, Watcher};

use super::DirWatcher;

pub struct LinuxWatcher {
    watcher: INotifyWatcher,
}

impl DirWatcher for LinuxWatcher {
    fn create(path: &PathBuf) -> (Receiver<RawEvent>, Self) {
        let (sender, receiver) = channel();
        let mut watcher = raw_watcher(sender).unwrap();
        watcher
            .watch(path, notify::RecursiveMode::NonRecursive)
            .unwrap();

        (receiver, Self { watcher })
    }
}
