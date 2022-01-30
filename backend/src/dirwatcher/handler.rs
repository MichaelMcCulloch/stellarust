use crate::dirwatcher::DirWatcher;
use data_model::CustodianMsg;
use notify::{Op, RawEvent};
use std::{
    fs,
    path::PathBuf,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::data_import::DataImport;

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
            log::info!("Discovered {:?}", path.file_name().unwrap());
            let parse_result = DataImport::from_file(&path);
            match parse_result {
                Ok(data_point) => custodian_message_sender
                    .send(CustodianMsg::Data(data_point))
                    .unwrap(),
                Err(e) => log::error!("{}", e),
            };
        }

        me.start_directory_event_handler(raw_event_receiver, custodian_message_sender);
        (custodian_message_receiver, me)
    }

    fn start_directory_event_handler(
        &self,
        raw_event_receiver: Receiver<RawEvent>,
        custodian_message_sender: Sender<CustodianMsg>,
    ) {
        thread::spawn(move || -> () {
            loop {
                match raw_event_receiver.recv() {
                    Err(_error) => break,
                    Ok(RawEvent {
                        op: Ok(Op::CLOSE_WRITE),
                        path: Some(path),
                        cookie: _cookie,
                    }) => {
                        match DataImport::from_file(&path) {
                            Ok(data) => {
                                match custodian_message_sender.send(CustodianMsg::Data(data)) {
                                    Ok(_) => {}
                                    Err(_) => {
                                        log::warn!(
                                            "Error sending data {:?}",
                                            &path.file_name().unwrap()
                                        )
                                    }
                                }
                            }
                            Err(_) => {
                                log::warn!("Error parsing {:?}", &path.file_name().unwrap())
                            }
                        }
                        continue;
                    }
                    _ => continue,
                }
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
