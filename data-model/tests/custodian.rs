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

    use data_model::{CustodianMsg, EmpireData, ModelCustodian, ModelDataPoint};
    use std::{sync::mpsc::channel, thread, time::Duration};

    #[test]
    fn test_custodian() {
        let (sender, receiver) = channel();

        let model = ModelCustodian::create(receiver);

        let s = sender.clone();
        thread::spawn(move || {
            s.send(get_custodian_message("0")).unwrap();
            s.send(get_custodian_message("2")).unwrap();
            s.send(get_custodian_message("3")).unwrap();
            s.send(get_custodian_message("6")).unwrap();
            s.send(CustodianMsg::Exit).unwrap();
        });

        thread::sleep(Duration::from_millis(500));

        let actual = model.get_campaign_data().unwrap();

        assert_eq!(
            actual,
            vec![
                get_model_data_point("0"),
                get_model_data_point("2"),
                get_model_data_point("3"),
                get_model_data_point("6"),
            ]
        );
    }

    fn get_custodian_message(name: &str) -> CustodianMsg {
        CustodianMsg::Data(get_model_data_point(name))
    }

    fn get_model_data_point(name: &str) -> ModelDataPoint {
        ModelDataPoint {
            empires: vec![EmpireData {
                name: String::from(name),
                resources: data_model::Resources {
                    energy: 0f64,
                    minerals: 0f64,
                    food: 0f64,
                    physics_research: 0f64,
                    society_research: 0f64,
                    engineering_research: 0f64,
                    influence: 0f64,
                    unity: 0f64,
                    consumer_goods: 0f64,
                    alloys: 0f64,
                    volatile_motes: 0f64,
                    exotic_gases: 0f64,
                    rare_crystals: 0f64,
                    sr_living_metal: 0f64,
                    sr_zro: 0f64,
                    sr_dark_matter: 0f64,
                },
            }],
        }
    }
}
