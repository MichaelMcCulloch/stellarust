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

    #[wasm_bindgen_test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
