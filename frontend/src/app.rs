use crate::{
    fetch::FetchState,
    route::{switch, Route},
};

use yew::{html, Component, Html};
type AppData = Vec<u32>;
use yew_router::prelude::*;

const APPDATA_URL: &str = "http://localhost:8000/";

pub struct AppState {
    data: FetchState<AppData>,
}

pub struct App {
    state: AppState,
}
pub enum Msg {}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        App::new()
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
        App::view_app()
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
        true
    }
}

impl App {
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

        assert_eq!(&result, "done");
    }
}
