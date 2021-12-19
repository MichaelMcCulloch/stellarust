use crate::fetch::{Fetch, FetchError, FetchState};
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

impl Fetch<SaveGameSelectData> for SaveGameSelect {}

impl SaveGameSelect {
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
                    <li class="" >{empire}</li>
                }
            })
            .collect::<Html>();

        html! {
            <li class="select-item">
                    <label class="" >
                        {game_dto.save_name.clone()}
                    </label>
                    <ul class="" >
                        {empires_html}
                    </ul>
                    <label class="" >
                        {game_dto.last_save_zoned_date_time}
                    </label>
            </li>
        }
    }

    fn view_save_game_select_list(save_game_dtos: &Vec<SaveGameDto>) -> Html {
        let save_game_dtos_html = save_game_dtos
            .into_iter()
            .map(|dto| SaveGameSelect::view_save_game_dto(dto))
            .collect::<Html>();

        html! {
            <ul>
                {save_game_dtos_html
            }</ul>
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
                SaveGameSelect::view_save_game_select_list(save_game_dtos)
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
}
