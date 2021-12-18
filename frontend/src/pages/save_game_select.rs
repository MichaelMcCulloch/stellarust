use crate::fetch::{FetchError, FetchState};
use stellarust::dto::SaveGameDto;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, Html};

type SaveGameSelectData = Vec<SaveGameDto>;

const SAVE_GAME_SELECT_DATA_URL: &str = "http://localhost:8000/saves";

pub struct SaveGameSelectState {
    empire_list: FetchState<SaveGameSelectData>,
}

pub struct SaveGameSelect {
    state: SaveGameSelectState,
}

pub enum Msg {
    GetEmpireList,
    SetFetchState(FetchState<SaveGameSelectData>),
}

impl Component for SaveGameSelect {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let save_game_select = SaveGameSelect::new();
        ctx.link().callback(|_: ()| Msg::GetEmpireList).emit(());
        save_game_select
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        SaveGameSelect::view_save_game_select(&self.state.empire_list)
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetEmpireList => {
                ctx.link().send_future(async {
                    match SaveGameSelect::fetch_data(SAVE_GAME_SELECT_DATA_URL).await {
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

impl SaveGameSelect {
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

    async fn fetch_data(url: &str) -> Result<SaveGameSelectData, FetchError> {
        let response_text = SaveGameSelect::process_request(
            gloo_utils::window(),
            SaveGameSelect::create_request_from_url(url).expect("Couldn't create Request"),
        )
        .await
        .expect("Couldn't process Request");

        let string = response_text
            .as_string()
            .expect("Couldnt get text from Response");
        let data: SaveGameSelectData = serde_json::from_str(string.as_str()).expect(&format!(
            "Could not deserialize '{}' to JSON",
            string.as_str()
        ));

        Ok(data)
    }

    fn new() -> Self {
        Self {
            state: SaveGameSelectState {
                empire_list: FetchState::NotFetching,
            },
        }
    }

    fn view_save_game_dto(game_dto: &SaveGameDto) -> Html {
        let empires = game_dto.empires.as_slice();

        let empires_html = empires
            .into_iter()
            .map(|empire| {
                html! {
                    <li class="" id="">{empire}</li>
                }
            })
            .collect::<Html>();

        html! {
            <li class="" id="">
                <div class="" id="">
                    <ul class="" id="">
                        {empires_html}
                    </ul>
                    <label class="" id="">
                        {game_dto.last_save_zoned_date_time}
                    </label>
                </div>
            </li>
        }
    }

    fn view_save_game_select(state: &FetchState<SaveGameSelectData>) -> Html {
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
            FetchState::Success(save_game_dtos) => {
                let cards = save_game_dtos
                    .into_iter()
                    .map(|dto| SaveGameSelect::view_save_game_dto(dto))
                    .collect::<Html>();
                cards
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
    use std::vec;

    use super::*;
    use time::macros::datetime;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use yew::html;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn save_game_select__view_save_game_select__not_fetching_label_not_fetching() {
        let html = SaveGameSelect::view_save_game_select(&FetchState::NotFetching);

        assert_eq!(
            html,
            html! {<label id="fetch-status not-fetching" class="fetch-status not-fetching">
                 {"Not Fetching"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn save_game_select__view_save_game_select__fetching_label_fetching() {
        let html = SaveGameSelect::view_save_game_select(&FetchState::Fetching);

        assert_eq!(
            html,
            html! {<label id="fetch-status fetching" class="fetch-status fetching">
                 {"Fetching"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn save_game_select__view_save_game_select__error_label_error() {
        let html = SaveGameSelect::view_save_game_select(&FetchState::Failed(FetchError::from(
            JsValue::from("_"),
        )));

        assert_eq!(
            html,
            html! {<label id="fetch-status failed" class="fetch-status failed">
                {"Error"}
            </label>}
        )
    }

    #[wasm_bindgen_test]
    async fn save_game_select__view_save_game_dto__label() {
        let save_game_dto = SaveGameDto {
            empires: vec![
                String::from("The Great Khanate"),
                String::from("Something Or Other"),
                String::from("A Third Thing"),
            ],
            last_save_zoned_date_time: datetime!(2021-12-25 0:00 UTC),
        };

        let empires_html = save_game_dto
            .empires
            .as_slice()
            .into_iter()
            .map(|empire| {
                html! {
                    <li class="" id="">{empire}</li>
                }
            })
            .collect::<Html>();

        let expected = html! {
            <li class="" id="">
                <div class="" id="">
                    <ul class="" id="">
                        {empires_html}
                    </ul>
                    <label class="" id="">
                        {save_game_dto.last_save_zoned_date_time}
                    </label>
                </div>
            </li>
        };

        let html = SaveGameSelect::view_save_game_dto(&save_game_dto);

        assert_eq!(html, expected)
    }
}
