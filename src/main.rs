use log::info;

use crate::scrapers::climatico::ClimaticoScraper;

pub mod scrapers;
pub mod model;

/// Initialize application state before startup.
fn init() {
    env_logger::init();
    info!("Application initialized.")
}

#[tokio::main]
async fn main() -> Result<(), String> {
    init();

    //    let mut climatico_scraper = ClimaticoScraper::new(
    //        "./out/climatico/sources/ac_residential",
    //        "./out/climatico/product_info/ac_residential",
    //    );

    //    climatico_scraper
    //        .save_page_sources("https://www.climatico.ro/aer-conditionat/vrv")
    //        .await?;

    ClimaticoScraper::extract_ac_product(
        "./out/climatico/sources/ac_residential",
        "./out/climatico/product_info/ac_residential",
    )
    .await?;

    //    climatico_scraper.close_session().await?;

    info!("Terminating application.");

    Ok(())
}
