use crate::fetch::{Fetch, FetchError, FetchState};
use stellarust::dto::CampaignDto;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, Html};

type CampaignSelectData = Vec<CampaignDto>;

const SAVE_GAME_SELECT_DATA_URL: &str = "http://localhost:8000/campaigns";

pub struct CampaignSelectState {
    campaigns_fetch_state: FetchState<CampaignSelectData>,
}

pub struct CampaignSelect {
    state: CampaignSelectState,
}

pub enum Msg {
    GetEmpireList,
    SetFetchState(FetchState<CampaignSelectData>),
}

impl Component for CampaignSelect {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        let campaign_select = CampaignSelect::new();
        ctx.link().callback(|_: ()| Msg::GetEmpireList).emit(());
        campaign_select
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        CampaignSelect::view_save_game_select(&self.state.campaigns_fetch_state)
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetEmpireList => {
                ctx.link().send_future(async {
                    match CampaignSelect::fetch_data(SAVE_GAME_SELECT_DATA_URL).await {
                        Ok(app_data) => Msg::SetFetchState(FetchState::Success(app_data)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            }
            Msg::SetFetchState(fetch_state) => {
                self.state.campaigns_fetch_state = fetch_state;
                true
            }
        }
    }
}

impl Fetch<CampaignSelectData> for CampaignSelect {}

impl CampaignSelect {
    fn new() -> Self {
        Self {
            state: CampaignSelectState {
                campaigns_fetch_state: FetchState::NotFetching,
            },
        }
    }

    fn view_save_game_dto(campaign: &CampaignDto) -> Html {
        let empires = campaign.empires.as_slice();

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
                        {campaign.save_name.clone()}
                    </label>
                    <ul class="" >
                        {empires_html}
                    </ul>
                    <label class="" >
                        {campaign.last_save_zoned_date_time}
                    </label>
            </li>
        }
    }

    fn view_save_game_select_list(campaigns: &Vec<CampaignDto>) -> Html {
        let campaigns_html = campaigns
            .into_iter()
            .map(|campaign| CampaignSelect::view_save_game_dto(campaign))
            .collect::<Html>();

        html! {
            <ul>
                {campaigns_html}
            </ul>
        }
    }

    fn view_save_game_select(state: &FetchState<CampaignSelectData>) -> Html {
        match state {
            FetchState::NotFetching => html! {
                <label class="fetch-info">
                     {"Not Fetching"}
                </label>
            },
            FetchState::Fetching => html! {
                <label class="fetch-info">
                    {"Fetching"}
                </label>
            },
            FetchState::Success(campaigns) => CampaignSelect::view_save_game_select_list(campaigns),
            FetchState::Failed(_) => html! {
                <label class="fetch-info">
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
