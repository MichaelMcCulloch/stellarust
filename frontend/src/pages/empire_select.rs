use crate::fetch::{Fetch, FetchState};
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

impl Fetch<EmpireSelectData> for EmpireSelect {}

impl EmpireSelect {
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
    use crate::fetch::FetchError;
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
