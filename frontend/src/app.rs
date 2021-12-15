use yew::{html, Component};

pub struct App {
    str: String,
}
pub enum Msg {
    Change,
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            str: String::from("Hello World!"),
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
        html! {
            <>
                <button
                    class="whatever"
                    onclick={ctx.link().callback(|_| Msg::Change)}>
                    {"Click Me!"}
                </button>
                <p class="label"> {&self.str} </p>
            </>
        }
    }
    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        todo!()
    }
    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change => {
                self.str = String::from("Goodbye World");
                true
            }
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
}
