use std::{
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use notify::{Op, RawEvent};

#[cfg(target_os = "linux")]
use crate::dirwatcher::linux::LinuxWatcher as DirectoryWatcher;
use crate::{dirwatcher::DirWatcher, model::CustodianMsg};

pub struct DirectoryEventHandler {
    watcher: DirectoryWatcher,
}

impl DirectoryEventHandler {
    pub fn create(directory: &PathBuf) -> (Receiver<CustodianMsg>, Self) {
        let (raw_event_receiver, watcher) = DirectoryWatcher::create(directory);
        let (custodian_message_sender, custodian_message_receiver) = channel::<CustodianMsg>();

        let me = DirectoryEventHandler { watcher };
        me.start_directory_event_handler(raw_event_receiver, custodian_message_sender);
        (custodian_message_receiver, me)
    }

    fn start_directory_event_handler(
        &self,
        raw_event_receiver: Receiver<RawEvent>,
        pathbuf_sender: Sender<CustodianMsg>,
    ) {
        thread::spawn(move || loop {
            match forward_event_to_path(&raw_event_receiver, &pathbuf_sender) {
                Err(e) => log::error!("{}", e),
                _ => continue,
            }
        });
    }
}

fn forward_event_to_path(
    raw_event_receiver: &Receiver<RawEvent>,
    pathbuf_sender: &Sender<CustodianMsg>,
) -> Result<(), anyhow::Error> {
    let event = raw_event_receiver.recv()?;

    (match event {
        RawEvent {
            op: Ok(Op::CLOSE_WRITE),
            path: Some(_path),
            cookie: _cookie,
        } => pathbuf_sender.send(CustodianMsg::Data(0)),
        _ => Ok(()),
    })?;

    Ok(())
}
