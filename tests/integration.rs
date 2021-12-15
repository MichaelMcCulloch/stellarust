#[cfg(test)]
mod tests {
    use anyhow::Result;
    use fantoccini::{Client, ClientBuilder, Locator};
    use std::sync::Once;

    struct TestState {
        client: Option<Client>,
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn button_click() -> Result<()> {
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
        client.close_window().await.unwrap();
        client.close().await.unwrap();
        Ok(())
    }
}
