use yew::{html, Component, Context, Html};

use crate::fetch::FetchState;

#[derive(Clone)]
pub struct AppState {
    data: FetchState<Vec<u32>>,
}

pub struct App {
    state: AppState,
}
pub enum Msg {
    Change,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &yew::Context<Self>) -> Self {
        App::new()
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        App::view_app()
    }
    fn changed(&mut self, _ctx: &yew::Context<Self>) -> bool {
        todo!()
    }
    fn update(&mut self, _ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change => true,
        }
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
            <div>
                <label id="number" class="number">{"0"}</label>
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::{html, FunctionComponent, FunctionProvider, Properties};

    trait TestAppState {
        fn getState(&self) -> AppState;
    }

    impl TestAppState for App {
        fn getState(&self) -> AppState {
            self.state.clone()
        }
    }

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
    fn it_works() {
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

        assert_eq!(result.as_str(), "done");
    }

    #[wasm_bindgen_test]
    fn view_shows_0() {
        yew::start_app_with_props_in_element::<App>(
            gloo_utils::document().get_element_by_id("output").unwrap(),
            (),
        );

        let number_field = gloo_utils::document()
            .get_element_by_id("number")
            .expect("No field with id 'number'");

        let field_text = number_field.text_content().unwrap();

        assert_eq!(field_text.as_str(), "0")
    }

    #[wasm_bindgen_test]
    fn app_create_state_is_not_fetching() {
        let app = App::new();

        let data = app.getState().data;

        assert_eq!(data, FetchState::NotFetching);
    }
}
