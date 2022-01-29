use actix_cors::Cors;
use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::{campaign_select::selector::CampaignSelector, dirwatcher::DirectoryEventHandler};
use data_model::ModelCustodian;
use listenfd::ListenFd;
use std::{panic, path::PathBuf, process::exit};

#[get("/empires")]
pub async fn empires(model_custodian: Data<ModelCustodian>) -> impl Responder {
    let names = model_custodian
        .get_ref()
        .clone()
        .get_empire_names()
        .await
        .expect("Could not get empire names");

    HttpResponse::Ok().json(names)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let _campaign_path = if let Some(arg) = args.get(1) {
        PathBuf::from(arg)
    } else {
        match CampaignSelector::select() {
            Ok(path) => path,
            Err(error) => {
                println!("{:?}", error);
                exit(-1)
            }
        }
    };

    let (receiver, _dir_watcher) = DirectoryEventHandler::create(&_campaign_path);
    let custodian = Data::new(ModelCustodian::create(receiver));

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(custodian.clone())
            .service(empires)
    });

    server = if let Some(listener) = ListenFd::from_env().take_tcp_listener(0)? {
        log::info!("{:?}", listener);
        server.listen(listener)?
    } else {
        panic!()
    };

    server.run().await
}

//These are int tests in disguise.
#[cfg(test)]
mod tests {

    use std::sync::mpsc::channel;

    use crate::empires;
    use actix_web::{body::Body, test, web::Data, App};
    use data_model::{Budget, CustodianMsg, EmpireData, ModelCustodian, ModelDataPoint, Resources};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_empires__from_custodian__returns_list_of_empire_names() {
        let expected_empire_names = vec![String::from("NAME")];

        let (sender, receiver) = channel();

        sender
            .send(CustodianMsg::Data(ModelDataPoint {
                campaign_name: String::new(),
                empires: vec![EmpireData {
                    name: String::from("NAME"),
                    budget: Budget::default(),
                    resources: Resources::default(),
                }],
            }))
            .unwrap();

        let custodian = ModelCustodian::create(receiver);

        let mut app =
            test::init_service(App::new().app_data(Data::new(custodian)).service(empires)).await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/empires")
            .to_request();

        let mut resp = test::call_service(&mut app, req).await;

        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());
        assert_eq!(&Body::from(json!(expected_empire_names.clone())), body);
    }
}
