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
            match receiver.recv() {
                Ok(data) => match data {
                    CustodianMsg::Data(i) => {
                        let x: Vec<_> = i
                            .clone()
                            .empires
                            .into_iter()
                            .map(|empire| empire.name)
                            .collect();
                        log::info!("{:?}", x);

                        history.lock().unwrap().push(i)
                    }
                    CustodianMsg::Exit => break,
                },
                _err => break,
            };
        });
    }

    pub fn get_empire_names(&self) -> Result<Vec<String>> {
        let empires = self.history.lock().unwrap().last().unwrap().empires.clone();

        let names = empires.into_iter().map(|empire| empire.name).collect();

        Ok(names)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn get_campaign_data__given_no_data__returns_empty_list() {}
}
