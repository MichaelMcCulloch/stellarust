use crate::pages::{empire_select::EmpireSelect, save_game_select::SaveGameSelect};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    SaveGameSelect,
    #[at("/empires")]
    EmpireSelect,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::EmpireSelect => {
            html! { <EmpireSelect /> }
        }
        Route::SaveGameSelect => {
            html! { <SaveGameSelect/>}
        }
    }
}
