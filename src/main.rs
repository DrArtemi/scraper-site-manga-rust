mod manga_scraper;
mod database;
mod models;
mod schema;

use manga_scraper::spiders;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    //  Init scraper
    let ms = manga_scraper::MangaScraper::new().await;
    ms.scrap_spider(&spiders::Spider::MangaPlus(spiders::mangaplus::MangaPlus::new())).await;
    ms.close().await
}
