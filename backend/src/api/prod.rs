use actix_web::{get, web::Data, Responder};
use data_model::ModelCustodian;

use crate::api::implementation::empires_impl;

#[get("/empires")]
pub async fn empires(model_custodian: Data<ModelCustodian>) -> impl Responder {
    empires_impl(model_custodian).await
}
