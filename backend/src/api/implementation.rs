use actix_web::{web::Data, HttpResponse, Responder};
use data_core::DataCoreBackend;
use data_model::ModelCustodian;

pub async fn empires_impl<D: DataCoreBackend>(
    model_custodian: Data<ModelCustodian<D>>,
) -> impl Responder {
    let names = model_custodian
        .get_ref()
        .clone()
        .get_empire_names()
        .await
        .expect("Could not get empire names");

    HttpResponse::Ok().json(names)
}
