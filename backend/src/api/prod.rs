use actix_web::{get, web::Data, HttpResponse, Responder};
use data_core::DataCore;
use data_model::ModelCustodian;
use std::sync::Mutex;

use crate::api::implementation::empires_impl;
use crate::broadcaster::Broadcaster;

#[get("/empires")]
pub async fn empires(model_custodian: Data<ModelCustodian<DataCore>>) -> impl Responder {
    empires_impl(model_custodian).await
}

#[get("/events")]
pub async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let client = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(client)
}
