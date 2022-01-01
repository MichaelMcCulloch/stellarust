use crate::dirwatcher::DirWatcher;
use model::CustodianMsg;
use notify::{Op, RawEvent};
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::parser::Parser;

#[cfg(target_os = "linux")]
use crate::dirwatcher::linux::LinuxWatcher as DirectoryWatcher;

pub struct DirectoryEventHandler {
    watcher: DirectoryWatcher,
}

impl DirectoryEventHandler {
    pub fn create(directory: &PathBuf) -> (Receiver<CustodianMsg>, Self) {
        let (raw_event_receiver, watcher) = DirectoryWatcher::create(directory);
        let (custodian_message_sender, custodian_message_receiver) = channel::<CustodianMsg>();

        let me = DirectoryEventHandler { watcher };

        let existant_files = get_existing_files(directory);

        for path in existant_files {
            custodian_message_sender
                .send(CustodianMsg::Data(Parser::from_file(&path)))
                .unwrap();
        }

        me.start_directory_event_handler(raw_event_receiver, custodian_message_sender);
        (custodian_message_receiver, me)
    }

    fn start_directory_event_handler(
        &self,
        raw_event_receiver: Receiver<RawEvent>,
        custodian_message_sender: Sender<CustodianMsg>,
    ) {
        thread::spawn(move || loop {
            match forward_event_to_path(&raw_event_receiver, &custodian_message_sender) {
                Err(e) => log::error!("{}", e),
                _ => continue,
            }
        });
    }
}

fn get_existing_files(path: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .into_iter()
        .filter_map(|result| match result {
            Ok(dir_entry) => Some(dir_entry.path()),
            Err(_) => None,
        })
        .collect()
}

fn forward_event_to_path(
    receiver: &Receiver<RawEvent>,
    sender: &Sender<CustodianMsg>,
) -> Result<(), anyhow::Error> {
    let event = receiver.recv()?;

    (match event {
        RawEvent {
            op: Ok(Op::CLOSE_WRITE),
            path: Some(path),
            cookie: _cookie,
        } => sender.send(CustodianMsg::Data(Parser::from_file(&path))),
        _ => Ok(()),
    })?;

    Ok(())
}
