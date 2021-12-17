use crate::pages::empire_select::EmpireSelect;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    EmpireSelect,
}

pub fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::EmpireSelect => {
            html! { <EmpireSelect /> }
        }
    }
}
