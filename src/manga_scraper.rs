pub mod spiders;
pub mod scraping_tools;

use spiders::Spider;
use scraping_tools::Scraper;

pub struct MangaScraper {
    scraper: Scraper,
}

impl MangaScraper {
    pub async fn new() -> MangaScraper {
        let scraper = Scraper::new().await;

        MangaScraper {
            scraper,
        }
    }

    pub async fn scrap_spider(&self, spider: &Spider) {
        let spider = Spider::get_spider(spider).unwrap();
        spider.get_comic_urls(self.scraper.clone()).await;
    }

    pub async fn close(&self) -> Result<(), fantoccini::error::CmdError> {
        self.scraper.close().await
    }
}