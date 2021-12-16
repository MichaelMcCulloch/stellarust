#[cfg(test)]
mod tests {
    use anyhow::Result;
    use fantoccini::{Client, ClientBuilder, Locator};
    use std::process::{Child, Command, Stdio};

    const LOCALHOST: &str = "http://localhost";

    fn start_geckodriver(port: u32) -> Child {
        Command::new("geckodriver")
            .arg(format!("--port={}", port))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process")
    }
    async fn connect_client(port: u32) -> Result<Client> {
        let client = ClientBuilder::native()
            .connect(format!("{}:{}", LOCALHOST, port).as_str())
            .await?;
        Ok(client)
    }
    async fn setup(port: u32) -> Result<(Client, Child)> {
        let child = start_geckodriver(port);
        let mut client = connect_client(port).await?;
        client
            .goto(format!("{}:{}", LOCALHOST, 3000).as_str())
            .await?;

        Ok((client, child))
    }
    async fn teardown(client: &mut Client, child: &mut Child) -> Result<()> {
        client.close_window().await?;

        child.kill()?;

        Ok(())
    }

    #[tokio::test]
    async fn open_page_read_0_from_server() -> Result<()> {
        let (mut client, mut child) = setup(4444).await.unwrap();

        let mut button = client
            .wait()
            .for_element(Locator::Css(".fetch"))
            .await
            .expect("could not locate class 'fetch'");

        button.click().await.expect("couldn't click the button");

        let mut label = client
            .wait()
            .for_element(Locator::Css(".fetch-result"))
            .await
            .expect("could not locate class 'fetch-result'");

        let text = label.text().await?;

        assert_eq!(text, String::from("[0]"));
        teardown(&mut client, &mut child).await.unwrap();

        Ok(())
    }
}
