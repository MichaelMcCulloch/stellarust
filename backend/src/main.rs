use actix_cors::Cors;
use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use backend::file_reader::SaveFileReader;
use backend::savedata;
use listenfd::ListenFd;
use std::panic;
use stellarust::dto::SaveGameDto;

#[get("/saves")]
pub async fn saves(save_games: Data<Vec<SaveGameDto>>) -> impl Responder {
    let data = save_games.get_ref().clone();
    HttpResponse::Ok().json(data)
}

#[get("/empires")]
pub async fn empires(empire_list: Data<Vec<String>>) -> impl Responder {
    let data = empire_list.get_ref().clone();
    HttpResponse::Ok().json(data)
}

#[get("/")]
pub async fn index(vec_32: Data<Vec<i32>>) -> impl Responder {
    let data = vec_32.get_ref().clone();
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

    //empire list:= fileReader.get()

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .app_data(data.clone())
            .app_data(empire_list.clone())
            //.app_data(empire_list_from_files)
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

//These are int tests in disguise.
#[cfg(test)]
mod tests {

    use crate::{empires, index, saves};
    use actix_web::{body::Body, test, web::Data, App};
    use serde_json::json;
    use std::panic;
    use std::str;
    use stellarust::dto::SaveGameDto;
    use time::macros::datetime;

    #[actix_rt::test]
    async fn test_index__returns_json_vec0() {
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
    async fn test_empires__returns_list_of_empire_names() {
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

    #[actix_rt::test]
    async fn test_saves__returns_list_of_saves() {
        let save_objects = vec![SaveGameDto {
            save_name: "".into(),
            empires: vec![],
            last_save_zoned_date_time: datetime!(2021-12-25 0:00 UTC),
        }];

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(save_objects.clone()))
                .service(saves),
        )
        .await;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .uri("/saves")
            .to_request();

        let mut resp = test::call_service(&mut app, req).await;

        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());

        if let Body::Bytes(bytes) = body {
            let x = bytes.as_ref();

            let string = str::from_utf8(x).unwrap();
            let actual_dto: Vec<SaveGameDto> = serde_json::from_str(string).unwrap();
            assert_eq!(actual_dto, save_objects)
        } else {
            panic!("body was not bytes");
        }
    }
}
