#[cfg(test)]
mod tests {
    use anyhow::Result;
    use fantoccini::{ClientBuilder, Locator};
    use std::process::{Child, Command, Stdio};

    #[tokio::test]
    async fn button_click() -> Result<()> {
        let port = 4444;

        let mut child = start_geckodriver(port);

        let mut client = ClientBuilder::native()
            .connect(format!("http://localhost:{}", port).as_str())
            .await?;

        client.goto("http://localhost:3000").await?;

        let button = client.wait().for_element(Locator::Css(".whatever")).await?;

        button.click().await?;

        let mut label = client.wait().for_element(Locator::Css(".label")).await?;

        let text = label.text().await?;

        assert_eq!(text, String::from("Goodbye World"));
        client.close_window().await?;
        client.close().await?;

        child.kill()?;
        Ok(())
    }

    #[tokio::test]
    async fn button_click_2() -> Result<()> {
        let port = 5555;

        let mut child = start_geckodriver(port);

        let mut client = ClientBuilder::native()
            .connect(format!("http://localhost:{}", port).as_str())
            .await?;

        client.goto("http://localhost:3000").await?;

        let button = client.wait().for_element(Locator::Css(".whatever")).await?;

        button.click().await?;

        let mut label = client.wait().for_element(Locator::Css(".label")).await?;

        let text = label.text().await?;

        assert_eq!(text, String::from("Goodbye World"));
        client.close_window().await?;
        client.close().await?;

        child.kill()?;
        Ok(())
    }

    fn start_geckodriver(port: u32) -> Child {
        let child = Command::new("geckodriver")
            .arg(format!("--port={}", port))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        println!(
            "Started Process geckodriver with pid {} on port {}",
            child.id(),
            port
        );

        child
    }
}
