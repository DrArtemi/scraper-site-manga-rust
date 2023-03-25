pub mod mangaplus;
mod models;

use async_trait::async_trait;
use crate::manga_scraper::scraping_tools::Scraper;
use models::{Comic, Chapter};

#[async_trait]
pub trait SpiderTrait {
    async fn get_comic_urls(&self, scraper: Scraper) -> Vec<Comic>;
    async fn get_chapters_urls(&self, scraper: Scraper, comic_url: &str) -> Vec<Chapter>;
}

pub enum Spider {
    MangaPlus(mangaplus::MangaPlus)
}

impl Spider {
    pub fn get_spider(&self) -> Option<&dyn SpiderTrait> {
        match self {
            Self::MangaPlus(v) => Some(v),
            _ => None,
        }
    }    
}
