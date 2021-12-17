use actix_cors::Cors;
use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use std::panic;

#[get("/empires")]
pub async fn empires(player_data: Data<Vec<String>>) -> impl Responder {
    let data = player_data.get_ref().clone();
    HttpResponse::Ok().json(data)
}

#[get("/")]
pub async fn index(player_data: Data<Vec<i32>>) -> impl Responder {
    let data = player_data.get_ref().clone();
    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let data = Data::new(vec![0]);
    let empire_list = Data::new(vec![
        String::from("The Great Khanate"),
        String::from("The Federation Of The Planets"),
        String::from("The Borg"),
        String::from("Q"),
        String::from("123434"),
        String::from("!@##$$()(*&())"),
    ]);

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(data.clone())
            .app_data(empire_list.clone())
            .service(index)
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

#[cfg(test)]
mod tests {

    use crate::{empires, index};
    use actix_web::{body::Body, test, web::Data, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_index_returnsJsonVec0() {
        let vec_0 = vec![0];
        let mut app =
            test::init_service(App::new().app_data(Data::new(vec_0.clone())).service(index)).await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/")
            .to_request();

        let mut resp = test::call_service(&mut app, req).await;

        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());
        assert_eq!(&Body::from(json!(vec_0)), body);
    }

    #[actix_rt::test]
    async fn test_empires_returnsListOfEmpireNames() {
        let empire_names = vec![
            String::from("The Great Khanate"),
            String::from("The Federation Of The Planets"),
        ];
        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(empire_names.clone()))
                .service(empires),
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/empires")
            .to_request();

        let mut resp = test::call_service(&mut app, req).await;

        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());
        assert_eq!(&Body::from(json!(empire_names.clone())), body);
    }
}
