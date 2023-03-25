use fantoccini::{Locator};
use async_trait::async_trait;

use crate::manga_scraper::scraping_tools::Scraper;
use crate::spiders::models::{Comic, Chapter};

use super::SpiderTrait;

pub struct MangaPlus {
    url:  String,
}

impl MangaPlus {
    pub fn new() -> MangaPlus {
        MangaPlus {
            url: "https://mangaplus.shueisha.co.jp".to_string()
        }
    }
}

#[async_trait]
impl SpiderTrait for MangaPlus {
    async fn get_comic_urls(&self, scraper: Scraper) -> Vec<Comic> {
        let manga_list_url: String = format!("{}{}", self.url, "/manga_list/all");
        scraper.client.goto(&manga_list_url).await;

        // Wait page loading
        scraper.client.wait().for_element(Locator::Css("a.AllTitle-module_allTitle_1CIUC")).await;

        // Retrieve comic links elements
        let comic_links = scraper.client.find_all(Locator::Css("a.AllTitle-module_allTitle_1CIUC")).await.unwrap();
        let mut comics: Vec<Comic> = Vec::new();
        for comic_link in &comic_links {
            let title: String = comic_link
                .find(Locator::Css("p.AllTitle-module_title_20PzS")).await.unwrap()
                .text().await.unwrap();
            let comic_url_suffix: String = comic_link
                .attr("href").await.unwrap().expect(&format!("Comic link empty for {:?}.", title));
            let comic_url: String = format!("{}{}", self.url, comic_url_suffix);
            let cover_url: String = comic_link
                .find(Locator::Css("img.AllTitle-module_image_JIEI9")).await.unwrap()
                .attr("data-src").await.unwrap().expect(&format!("Comic cover url empty for {:?}.", title));
            comics.push(Comic {
                title: title,
                url: comic_url,
                cover_url: cover_url
            });
        }

        // Return object with url, cover url and title
        comics
    }

    async fn get_chapters_urls(&self, scraper: Scraper, comic_url: &str) -> Vec<Chapter> {
        scraper.client.goto(comic_url).await;

        // Wait page loading
        scraper.client.wait().for_element(Locator::Css("div.ChapterListItem-module_chapterListItem_ykICp")).await;

        let chapters_information = scraper.client.find_all(Locator::Css("div.ChapterListItem-module_chapterListItem_ykICp:not(.ChapterList-module_separator_1Oam1 *)")).await.unwrap();
        let mut chapters: Vec<Chapter> = Vec::new();
        for chapter_information in &chapters_information {
            let number: String = chapter_information
                .find(Locator::Css("p.ChapterListItem-module_name_3h9dj")).await.unwrap()
                .text().await.unwrap()
                .replace("#", "");
            let date: String = chapter_information
                .find(Locator::Css("p.ChapterListItem-module_date_xe1XF")).await.unwrap()
                .text().await.unwrap();
            let chapter_id: String = chapter_information
                    .find(Locator::Css("a.ChapterListItem-module_commentContainer_1P6qt")).await.unwrap()
                    .attr("href").await.unwrap().expect(&format!("Chapter url empty for {:?}.", number))
                    .rsplit_once('/').unwrap().1.to_string();
            let chapter_url = format!("{}/viewer/{}", self.url, chapter_id);
            chapters.push(Chapter {
                url: chapter_url,
                number: number,
                date: date
            });
        }

        chapters
    }
}