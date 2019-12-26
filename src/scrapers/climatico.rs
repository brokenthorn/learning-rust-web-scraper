//! A module for scraping `https://www.climatico.ro/`.

use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use fantoccini::{Client, Locator};
use log::{error, info};
use url::Url;

use crate::scrapers::url_to_html_file_name;

pub struct ClimaticoScraper<'a> {
    client: fantoccini::Client,
    /// Folder path for saving web page sources to disk.
    /// Very useful so web page don't need to be fetched every time
    /// when scraping new resources from them.
    page_sources_output_path: &'a Path,
    /// Folder path for saving scraped product information to disk.
    product_info_output_path: &'a Path,
}

impl<'a> Default for ClimaticoScraper<'a> {
    fn default() -> Self {
        info!("Creating ClimaticoScraper using default configuration.");

        let client_future = Client::new("http://localhost:4444");
        let client = match futures::executor::block_on(client_future) {
            Ok(c) => c,
            // TODO: display error message in panic.
            Err(_) => {
                panic!("Failed to create new WebDriver session with http://localhost:4444.");
            }
        };

        Self {
            client,
            page_sources_output_path: Path::new("./"),
            product_info_output_path: Path::new("./"),
        }
    }
}

impl<'a> ClimaticoScraper<'a> {
    pub fn new(page_sources_output_path: &'a str, product_info_output_path: &'a str) -> Self {
        info!("Creating ClimaticoScraper.");

        let client_future = Client::new("http://localhost:4444");
        let client = match futures::executor::block_on(client_future) {
            Ok(c) => c,
            // TODO: display error message in panic.
            Err(_) => {
                panic!("Failed to create new WebDriver session with http://localhost:4444.");
            }
        };

        Self {
            client,
            page_sources_output_path: Path::new(page_sources_output_path),
            product_info_output_path: Path::new(product_info_output_path),
        }
    }

    pub async fn save_page_sources(
        &mut self,
        first_page_url: &str,
    ) -> Result<(), fantoccini::error::CmdError> {
        info!("Saving page sources starting with {}", first_page_url);

        let mut page_url = Url::from_str(first_page_url)
            .expect("Failed to parse argument first_page_url into a valid URL.");

        // Navigate to each page of the product listing and save the pages to disk:
        loop {
            let source_file_pathbuf = match url_to_html_file_name(&page_url) {
                Ok(p) => self.page_sources_output_path.join(p),
                Err(e) => {
                    error!(
                        "Failed to determine path for source file from its URL: {}",
                        e
                    );
                    panic!(e);
                }
            };

            info!("Creating page_sources_output_path directory structure, if it's missing.");

            std::fs::create_dir_all(self.page_sources_output_path)
                .expect("Failed to create directory structure.");

            info!("Navigating to page {:?}", page_url);

            self.client.goto(page_url.as_ref()).await?;

            let source = self.client.source().await?;
            let mut source_file = std::fs::File::create(source_file_pathbuf.as_path())
                .expect("Failed to create file for page source.");

            info!("Writing source file to disk: {:?}", source_file_pathbuf);

            source_file
                .write_all(source.as_ref())
                .expect("Failed to write page source to disk.");

            match self
                .client
                .find(Locator::Css("head > link[rel=next]"))
                .await
            {
                Ok(mut link) => {
                    let link_url = link
                        .attr("href")
                        .await?
                        .expect("link tag with rel=next should have an href attribute!");

                    info!("Found next page at {}", link_url);

                    page_url = Url::from_str(link_url.as_str()).unwrap();
                }
                Err(_) => {
                    info!("No more pages left.");

                    break;
                }
            }
        }

        Ok(())
    }
}

//impl ClimaticoScraper {
//    /// Start scraping an entire product listing, starting at the page specified by the
//    /// `product_listing_page_url` argument.
//    ///
//    /// __NOTE__: To determine the next page of the product listing, this function
//    /// expects to find a `link` tag in the HTML header that points to the next URL.
//    /// For example `<link rel="next" href="https://...?p=2">`.
//    pub async fn scrape_product_listing(product_listing_page_url: &str) -> Result<u64, ()> {
//        let current_url = product_listing_page_url;
//
//        loop {
//            info!("Scraping {}", current_url);
//            break;
//        }
//
//        /*let client = HttpClient::builder()
//            .timeout(Duration::from_secs(10))
//            .redirect_policy(RedirectPolicy::Limit(10))
//            .version_negotiation(VersionNegotiation::http11())
//            .build()?;
//
//        client
//            .get_async("https://www.yahoo.com/")
//            .await?
//            .copy_to_file("./test.html")*/
//
//        Ok(0)
//    }
//}
