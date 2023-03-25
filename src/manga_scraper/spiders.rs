use fantoccini::{Locator};
use async_trait::async_trait;
use crate::manga_scraper::scraping_tools::Scraper;

#[async_trait]
pub trait SpiderTrait {
    async fn get_comic_urls(&self, scraper: Scraper);
}

// MANGAPLUS

pub struct MangaPlus {
    url:  String,
}

impl MangaPlus {
    pub fn new() -> MangaPlus {
        MangaPlus {
            url: "https://mangaplus.shueisha.co.jp/manga_list/all".to_string()
        }
    }
}

#[async_trait]
impl SpiderTrait for MangaPlus {
    async fn get_comic_urls(&self, scraper: Scraper) {
        scraper.client.goto(&self.url).await;
        let url = scraper.client.current_url().await.unwrap();
        assert_eq!(url.as_str(), self.url);
        println!("{:?}", url.as_str());

        // Wait page loading
        scraper.client.wait().for_element(Locator::Css("a.AllTitle-module_allTitle_1CIUC")).await;

        // Retrieve comic links elements
        let comic_links = scraper.client.find_all(Locator::Css("a.AllTitle-module_allTitle_1CIUC")).await.unwrap();
        for comic_link in  comic_links.iter() {
            println!("{:?}", comic_link.attr("href").await);
        }
    }
}

pub enum Spider {
    MangaPlus(MangaPlus)
}

impl Spider {
    pub fn get_spider(&self) -> Option<&dyn SpiderTrait> {
        match self {
            Self::MangaPlus(v) => Some(v),
            _ => None,
        }
    }    
}
