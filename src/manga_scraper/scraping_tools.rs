use fantoccini::{ClientBuilder, Client};

#[derive(Clone)]
pub struct Scraper {
    pub client: Client,
}

impl Scraper {
    pub async fn new() -> Scraper {
        // Setup firefox options
        let mut caps = serde_json::map::Map::new();
        let opts = serde_json::json!({"args": ["--headless"]});
        caps.insert("moz:firefoxOptions".to_string(), opts.clone());
    
        // Build browser client
        let client = ClientBuilder::native()
        .capabilities(caps)
        .connect("http://localhost:4444").await.expect("failed to connect to WebDriver");

        // Return built Scraper structure
        Scraper {
            client
        }
    }

    pub async fn close(&self) -> Result<(), fantoccini::error::CmdError> {
        // Closes WebDriver
        self.client.clone().close().await
    }
}