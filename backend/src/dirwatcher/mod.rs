mod handler;
#[cfg(target_os = "linux")]
mod linux;

pub use handler::DirectoryEventHandler;
use notify::RawEvent;
use std::{path::PathBuf, sync::mpsc::Receiver};

pub trait DirWatcher {
    fn create(path: &PathBuf) -> (Receiver<RawEvent>, Self);
}
