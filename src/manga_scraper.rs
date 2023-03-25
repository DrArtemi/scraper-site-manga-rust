pub mod spiders;
pub mod scraping_tools;

use diesel::prelude::*;
use spiders::Spider;
use scraping_tools::Scraper;
use crate::database::*;

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
        let db_connection = &mut connect_db();

        let spider = Spider::get_spider(spider).unwrap();
        let comics = spider.get_comic_urls(self.scraper.clone()).await;
        for comic in &comics {
            // Save comic information if it doesn't already exist
            create_comic( db_connection, &comic.title, &comic.url, &comic.cover_url);
            let chapters = spider.get_chapters_urls(self.scraper.clone(), &comic.url).await;
            for chapter in &chapters {
                create_chapter( db_connection, &chapter.url, &chapter.number, &chapter.date);
            }
        }
    }

    pub async fn close(&self) -> Result<(), fantoccini::error::CmdError> {
        self.scraper.close().await
    }
}