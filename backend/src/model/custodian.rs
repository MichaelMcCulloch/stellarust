use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
};

use anyhow::Result;

use super::data::ModelDataPoint;

pub struct ModelCustodian {
    history: Arc<Mutex<Vec<ModelDataPoint>>>,
}

#[derive(Debug, PartialEq)]
pub enum CustodianMsg {
    Data(ModelDataPoint),
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

    pub fn get_campaign_data(&self) -> Result<Vec<ModelDataPoint>> {
        Ok(self.history.lock().unwrap().clone())
    }
}

#[cfg(test)]
mod tests {}
