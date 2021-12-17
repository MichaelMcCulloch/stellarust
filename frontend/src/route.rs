use crate::pages::empire_select::EmpireSelect;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <EmpireSelect /> }
        }
    }
}
