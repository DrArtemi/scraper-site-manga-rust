pub mod spiders;
pub mod scraping_tools;
pub mod models;

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
        let comics = spider.get_comic_urls(self.scraper.clone()).await;
        for comic in &comics {
            // Save comic information if it doesn't already exist
            println!("{:?}", comic);
            let chapters = spider.get_chapters_urls(self.scraper.clone(), &comic.url).await;
            for chapter in &chapters {
                println!("{:?}", chapter);
            }
        }
    }

    pub async fn close(&self) -> Result<(), fantoccini::error::CmdError> {
        self.scraper.close().await
    }
}