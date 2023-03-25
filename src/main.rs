mod manga_scraper;

use manga_scraper::spiders;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    //  Init scraper
    let ms = manga_scraper::MangaScraper::new().await;
    ms.scrap_spider(&spiders::Spider::MangaPlus(spiders::MangaPlus::new())).await;
    ms.close().await
}
