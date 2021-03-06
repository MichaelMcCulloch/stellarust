use actix_web::{get, web::Data, Responder};
use data_model::ModelCustodian;

use crate::api::implementation::empires_impl;

#[get("/empires")]
pub async fn empires_test(model_custodian: Data<ModelCustodian>) -> impl Responder {
    empires_impl(model_custodian).await
}

#[cfg(test)]
mod api_tests {

    use std::sync::mpsc::channel;

    use actix_web::{body::Body, get, test, web::Data, App, Responder};
    use data_model::{Budget, CustodianMsg, EmpireData, ModelCustodian, ModelDataPoint, Resources};
    use serde_json::json;

    use super::empires_test;

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

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(custodian))
                .service(empires_test),
        )
        .await;
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
