use crate::{
    fetch::{FetchError, FetchState},
    route::{switch, Route},
};
use anyhow::Result;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{MouseEvent, Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, Context, Html};
type AppData = Vec<u32>;
use yew_router::prelude::*;

const APPDATA_URL: &str = "http://localhost:8000/";

pub struct AppState {
    data: FetchState<AppData>,
}

pub struct App {
    state: AppState,
}
pub enum Msg {
    GetData,
    SetFetchState(FetchState<AppData>),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        App::new()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        App::view_app()
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetData => {
                ctx.link().send_future(async {
                    match App::fetch_data(APPDATA_URL).await {
                        Ok(app_data) => Msg::SetFetchState(FetchState::Success(app_data)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::SetFetchState(fetch_state) => {
                self.state.data = fetch_state;
                true
            }
        }
    }
}

impl App {
    async fn process_request(window: Window, request: Request) -> Result<JsValue, JsValue> {
        let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = response_value
            .dyn_into()
            .expect("Couldnt get Response from Response Value");
        let text = JsFuture::from(resp.text()?)
            .await
            .expect("Couldn't get Response Text Content");

        Ok(text)
    }

    fn create_request_from_url(url: &str) -> Result<Request, JsValue> {
        let mut request_init = RequestInit::new();
        request_init.method("GET");
        request_init.mode(RequestMode::Cors);
        Request::new_with_str_and_init(url, &request_init)
    }

    async fn fetch_data(url: &str) -> Result<AppData, FetchError> {
        let response_text = App::process_request(
            gloo_utils::window(),
            App::create_request_from_url(url).expect("Couldn't create Request"),
        )
        .await
        .expect("Couldn't process Request");

        let string = response_text
            .as_string()
            .expect("Couldnt get text from Response");
        let data: AppData = serde_json::from_str(string.as_str()).expect(&format!(
            "Could not deserialize '{}' to JSON",
            string.as_str()
        ));
        Ok(data)
    }

    fn new() -> Self {
        Self {
            state: AppState {
                data: FetchState::NotFetching,
            },
        }
    }
    fn view_app() -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Route> render={Switch::render(switch)}/>
                </main>
            </BrowserRouter>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::{html, FunctionComponent, FunctionProvider, Properties};
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    pub fn obtain_result() -> String {
        gloo_utils::document()
            .get_element_by_id("result")
            .expect("No result found. Most likely, the application crashed and burned")
            .inner_html()
    }

    pub fn obtain_result_by_id(id: &str) -> String {
        gloo_utils::document()
            .get_element_by_id(id)
            .expect("No result found. Most likely, the application crashed and burned")
            .inner_html()
    }

    #[wasm_bindgen_test]
    async fn it_works() {
        struct PropsPassedFunction {}
        #[derive(Properties, Clone, PartialEq)]
        struct PropsPassedFunctionProps {
            value: String,
        }
        impl FunctionProvider for PropsPassedFunction {
            type TProps = PropsPassedFunctionProps;

            fn run(props: &Self::TProps) -> yew::Html {
                assert_eq!(&props.value, "props");
                html! {
                    <div id="result">
                        {"done"}
                    </div>
                }
            }
        }
        type PropsComponent = FunctionComponent<PropsPassedFunction>;

        yew::start_app_with_props_in_element::<PropsComponent>(
            gloo_utils::document().get_element_by_id("output").unwrap(),
            PropsPassedFunctionProps {
                value: "props".to_string(),
            },
        );

        let result = obtain_result();

        assert_eq!(result.as_str(), "done");
    }

    #[wasm_bindgen_test]
    async fn app_create_state_is_not_fetching() {
        let app = App::new();

        let data = app.state.data;

        assert_eq!(data, FetchState::NotFetching);
    }
}
