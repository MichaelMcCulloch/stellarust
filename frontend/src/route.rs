use crate::pages::{campaign_select::CampaignSelect, empire_select::EmpireSelect};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    CampaignSelect,
    #[at("/empires")]
    EmpireSelect,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::EmpireSelect => {
            html! { <EmpireSelect /> }
        }
        Route::CampaignSelect => {
            html! { <CampaignSelect/>}
        }
    }
}
