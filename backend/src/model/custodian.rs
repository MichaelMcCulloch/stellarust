use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use anyhow::Result;

type ModelData = usize;

pub struct ModelCustodian {
    history: Arc<Mutex<Vec<ModelData>>>,
}

#[derive(Debug)]
pub enum CustodianMsg {
    Data(ModelData),
    Exit,
}

impl ModelCustodian {
    pub fn create(receiver: Receiver<CustodianMsg>) -> Self {
        let me = ModelCustodian {
            history: Arc::new(Mutex::new(vec![])),
        };
        me.start(receiver);
        me
    }

    fn start(&self, receiver: Receiver<CustodianMsg>) {
        let history = self.history.clone();
        thread::spawn(move || loop {
            match receiver.recv().unwrap() {
                CustodianMsg::Data(i) => history.lock().unwrap().push(i),
                CustodianMsg::Exit => break,
            };
        });
    }

    pub fn get_campaign_data(&self) -> Result<Vec<ModelData>> {
        Ok(self.history.lock().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {}
