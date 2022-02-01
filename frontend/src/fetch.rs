use async_trait::async_trait; //TODO wait for rustc to support async fn natively
use serde::de::DeserializeOwned;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[async_trait(?Send)]
pub trait Fetch<T>
where
    T: DeserializeOwned,
{
    async fn fetch_data(url: &str) -> Result<T, FetchError> {
        let mut request_init = RequestInit::new();
        request_init.method("GET");
        request_init.mode(RequestMode::Cors);
        let request =
            Request::new_with_str_and_init(url, &request_init).expect("Couldn't create Request");
        let window = gloo_utils::window();
        let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = response_value
            .dyn_into()
            .expect("Couldnt get Response from Response Value");
        let response_text = JsFuture::from(resp.text()?)
            .await
            .expect("Couldn't get Response Text Content");

        let string = response_text
            .as_string()
            .expect("Couldnt get text from Response");

        let data: T = serde_json::from_str(&string)
            .expect(&format!("Could not deserialize '{}' to JSON", string));
        Ok(data)
    }
}
