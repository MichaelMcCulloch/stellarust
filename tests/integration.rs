#[cfg(test)]
mod tests {
    use fantoccini::{ClientBuilder, Locator};
    use tokio::time::sleep;
    use wasm_bindgen_test::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn button_click() {
        let mut client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await
            .expect("Failed to connect to Webdriver");

        client
            .goto("http://localhost:3000")
            .await
            .expect("host page not found");

        let button = client
            .wait()
            .for_element(Locator::Css(".whatever"))
            .await
            .unwrap();

        button.click().await.unwrap();

        let mut label = client
            .wait()
            .for_element(Locator::Css(".label"))
            .await
            .unwrap();

        let text = label.text().await.unwrap();

        assert_eq!(text, String::from("Goodbye World"));
    }
}
