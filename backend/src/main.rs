use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::{
    api::empires, campaign_select::selector::CampaignSelector, dirwatcher::DirectoryEventHandler,
};
use data_core::DataCore;
use data_model::ModelCustodian;
use listenfd::ListenFd;
use std::{panic, path::PathBuf, process::exit};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let campaign_path = if let Some(arg) = args.get(1) {
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

    let (receiver, _dir_watcher) = DirectoryEventHandler::create(&campaign_path);

    let data_core = DataCore::create(&campaign_path, &"stellarust.db")
        .await
        .expect("Could not open Database");

    let custodian_data = Data::new(ModelCustodian::create(receiver, data_core));

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(custodian_data.clone())
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
