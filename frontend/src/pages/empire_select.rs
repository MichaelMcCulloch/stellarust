use crate::fetch::{FetchError, FetchState};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, Html};

type EmpireSelectData = Vec<String>;

const HOMEDATA_URL: &str = "http://localhost:8000/empires";

pub struct EmpireSelectState {
    empire_list: FetchState<EmpireSelectData>,
}

pub struct EmpireSelect {
    state: EmpireSelectState,
}

pub enum Msg {
    GetEmpireList,
    SetFetchState(FetchState<EmpireSelectData>),
}

impl Component for EmpireSelect {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let home = EmpireSelect::new();
        ctx.link().callback(|_: ()| Msg::GetEmpireList).emit(());
        home
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        EmpireSelect::view_home(&self.state.empire_list)
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetEmpireList => {
                ctx.link().send_future(async {
                    match EmpireSelect::fetch_data(HOMEDATA_URL).await {
                        Ok(app_data) => Msg::SetFetchState(FetchState::Success(app_data)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::SetFetchState(fetch_state) => {
                self.state.empire_list = fetch_state;
                true
            }
        }
    }
}

impl EmpireSelect {
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

    async fn fetch_data(url: &str) -> Result<EmpireSelectData, FetchError> {
        let response_text = EmpireSelect::process_request(
            gloo_utils::window(),
            EmpireSelect::create_request_from_url(url).expect("Couldn't create Request"),
        )
        .await
        .expect("Couldn't process Request");

        let string = response_text
            .as_string()
            .expect("Couldnt get text from Response");
        let data: EmpireSelectData = serde_json::from_str(string.as_str()).expect(&format!(
            "Could not deserialize '{}' to JSON",
            string.as_str()
        ));

        Ok(data)
    }

    fn new() -> Self {
        Self {
            state: EmpireSelectState {
                empire_list: FetchState::NotFetching,
            },
        }
    }
    fn view_home(state: &FetchState<EmpireSelectData>) -> Html {
        match state {
            FetchState::NotFetching => html! {
                <label id="fetch-status not-fetching" class="fetch-status not-fetching">
                     {"Not Fetching"}
                </label>
            },
            FetchState::Fetching => html! {
                <label id="fetch-status fetching" class="fetch-status fetching">
                    {"Fetching"}
                </label>
            },
            FetchState::Success(empire_list) => {
                let labels = empire_list
                    .into_iter()
                    .map(|f| {
                        html! {
                            <label id="empire-name" class="empire-name">
                            {f}
                        </label>}
                    })
                    .collect::<Html>();

                html! {<div id="fetch-status success" class="fetch-status success">{labels}</div>}
            }
            FetchState::Failed(_) => html! {
                <label id="fetch-status failed" class="fetch-status failed">
                    {"Error"}
                </label>
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use yew::html;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn home_view_home_not_fetching_label_not_fetching() {
        let html = EmpireSelect::view_home(&FetchState::NotFetching);

        assert_eq!(
            html,
            html! {<label id="fetch-status not-fetching" class="fetch-status not-fetching">
                 {"Not Fetching"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn home_view_home_fetching_label_fetching() {
        let html = EmpireSelect::view_home(&FetchState::Fetching);

        assert_eq!(
            html,
            html! {<label id="fetch-status fetching" class="fetch-status fetching">
                 {"Fetching"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn home_view_home_error_label_error() {
        let html =
            EmpireSelect::view_home(&FetchState::Failed(FetchError::from(JsValue::from("_"))));

        assert_eq!(
            html,
            html! {<label id="fetch-status failed" class="fetch-status failed">
                {"Error"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn home_view_home_success_div_list_empire_names() {
        let empires = vec![
            String::from("The Great Khanate"),
            String::from("The Federation Of The Planets"),
            String::from("The Borg"),
            String::from("Q"),
            String::from("123434"),
            String::from("!@##$$()(*&())"),
        ];

        let children = html! {<>
            <label id="empire-name" class="empire-name">
                {"The Great Khanate"}
            </label>
            <label id="empire-name" class="empire-name">
                {"The Federation Of The Planets"}
            </label>
            <label id="empire-name" class="empire-name">
                {"The Borg"}
            </label>
            <label id="empire-name" class="empire-name">
                {"Q"}
            </label>
            <label id="empire-name" class="empire-name">
                {"123434"}
            </label>
            <label id="empire-name" class="empire-name">
                {"!@##$$()(*&())"}
            </label>
        </>};

        let expected = html! {
            <div id="fetch-status success" class="fetch-status success">
                {children}
            </div>
        };

        let html = EmpireSelect::view_home(&FetchState::Success(empires));

        assert_eq!(html, expected)
    }
}
