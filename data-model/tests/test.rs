use std::path::PathBuf;

fn get_resource_dir() -> PathBuf {
    let test_resource_dir = {
        let mut dir: PathBuf = PathBuf::from(std::env::current_dir().unwrap());
        dir.pop();
        dir.push("res");
        dir.push("test_data");
        dir.push("campaign");
        dir.push("unitednationsofearth_-15512622");
        dir
    };
    test_resource_dir
}

#[cfg(test)]
mod tests {

    use data_model::{CustodianMsg, ModelCustodian, ModelDataPoint};
    use std::{sync::mpsc::channel, thread, time::Duration};

    #[test]
    fn test_model() {
        let (sender, receiver) = channel();

        let model = ModelCustodian::create(receiver);

        let s = sender.clone();
        thread::spawn(move || {
            s.send(CustodianMsg::Data(ModelDataPoint { data: 0 }))
                .unwrap();
            s.send(CustodianMsg::Data(ModelDataPoint { data: 2 }))
                .unwrap();
            s.send(CustodianMsg::Data(ModelDataPoint { data: 3 }))
                .unwrap();
            s.send(CustodianMsg::Data(ModelDataPoint { data: 6 }))
                .unwrap();
            s.send(CustodianMsg::Exit).unwrap();
        });

        thread::sleep(Duration::from_millis(500));

        let actual = model.get_campaign_data().unwrap();

        assert_eq!(
            actual,
            vec![
                ModelDataPoint { data: 0 },
                ModelDataPoint { data: 2 },
                ModelDataPoint { data: 3 },
                ModelDataPoint { data: 6 }
            ]
        );
    }
}
