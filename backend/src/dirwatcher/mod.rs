mod handler;
#[cfg(target_os = "linux")]
mod linux;

pub use handler::DirectoryEventHandler;
use notify::RawEvent;
use std::{path::Path, sync::mpsc::Receiver};

pub trait DirWatcher {
    fn create(path: &Path) -> (Receiver<RawEvent>, Self);
}
