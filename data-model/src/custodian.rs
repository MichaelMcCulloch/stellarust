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
                        history.lock().unwrap().push(i);
                        log::info!("Received New Data");
                    }
                    CustodianMsg::Exit => break,
                },
                _err => break,
            };
        });
    }

    pub async fn get_empire_names(&self) -> Result<Vec<String>> {
        match self.history.lock().unwrap().last() {
            Some(data_point) => {
                let empires = &data_point.empires;
                Ok(empires
                    .into_iter()
                    .map(|empire| empire.name.clone())
                    .collect())
            }
            None => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Budget, CustodianMsg, EmpireData, ModelCustodian, ModelDataPoint, Resources};
    use std::{sync::mpsc::channel, thread, time::Duration};

    const EMPIRE_NAME: &str = "EMPIRE_NAME";

    #[actix_rt::test]
    async fn get_empire_names__given_no_data__returns_empty_list() {
        let (sender, receiver) = channel();
        sender.send(CustodianMsg::Exit).unwrap();

        let model = ModelCustodian::create(receiver);

        thread::sleep(Duration::from_millis(5));

        let actual = (model.get_empire_names()).await.unwrap();

        assert!(actual.is_empty());
    }

    #[actix_rt::test]
    async fn get_empire_names__given_single_data__returns_name_from_data() {
        let (sender, receiver) = channel();
        sender.send(get_custodian_message(EMPIRE_NAME)).unwrap();
        sender.send(CustodianMsg::Exit).unwrap();
        let model = ModelCustodian::create(receiver);

        thread::sleep(Duration::from_millis(5));

        let actual = model.get_empire_names().await.unwrap();

        assert_eq!(actual, vec![String::from(EMPIRE_NAME),]);
    }

    #[actix_rt::test]
    async fn get_empire_names__given_series_of_data__returns_list_of_names_from_last_in_series() {
        let (sender, receiver) = channel();
        sender.send(get_custodian_message("0")).unwrap();
        sender.send(get_custodian_message("2")).unwrap();
        sender.send(get_custodian_message("3")).unwrap();
        sender.send(get_custodian_message(EMPIRE_NAME)).unwrap();
        sender.send(CustodianMsg::Exit).unwrap();
        let model = ModelCustodian::create(receiver);

        thread::sleep(Duration::from_millis(5));

        let actual = model.get_empire_names().await.unwrap();

        assert_eq!(actual, vec![String::from(EMPIRE_NAME),]);
    }

    fn get_custodian_message(empire_name: &str) -> CustodianMsg {
        CustodianMsg::Data(ModelDataPoint {
            campaign_name: String::from("The Great Campaign"),
            empires: vec![EmpireData {
                name: String::from(empire_name),
                resources: Resources::default(),
                budget: Budget::default(),
            }],
        })
    }
}
