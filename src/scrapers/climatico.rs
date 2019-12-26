//! A module for scraping `https://www.climatico.ro/`.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use fantoccini::{Client, Locator};
use log::{error, info};
use select::document::Document;
use select::predicate;
use url::Url;

use crate::scrapers::url_to_html_file_name;
use select::predicate::Predicate;

/// A web scraper for `https://www.climatico.ro/` that employs an internal WebDriver client.
///
/// The default instance or the one created with new(), connects the WebDriver client to
/// `http://localhost:4444` immediately and will panic if it cannot establish a session.
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
    /// Create a new ClimaticoScraper using default configuration values.
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

        let page_sources_output_path = "./out/";
        let product_info_output_path = "./out/";

        info!("Creating page sources output directory structure, if it's missing.");

        std::fs::create_dir_all(page_sources_output_path)
            .expect("Failed to create page sources output directory structure.");

        info!("Creating product info output directory structure, if it's missing.");

        std::fs::create_dir_all(product_info_output_path)
            .expect("Failed to create product info output directory structure.");

        Self {
            client,
            page_sources_output_path: Path::new(page_sources_output_path),
            product_info_output_path: Path::new(product_info_output_path),
        }
    }
}

impl<'a> ClimaticoScraper<'a> {
    /// Create a new ClimaticoScraper.
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

        info!("Creating page sources output directory structure, if it's missing.");

        std::fs::create_dir_all(page_sources_output_path)
            .expect("Failed to create page sources output directory structure.");

        info!("Creating product info output directory structure, if it's missing.");

        std::fs::create_dir_all(product_info_output_path)
            .expect("Failed to create product info output directory structure.");

        Self {
            client,
            page_sources_output_path: Path::new(page_sources_output_path),
            product_info_output_path: Path::new(product_info_output_path),
        }
    }

    /// Save page sources for an entire product listing, starting at [first_page_url].
    /// Automatically finds the next page and stops when it doesn't find any more pages.
    pub async fn save_page_sources(
        &mut self,
        first_page_url: &str,
    ) -> Result<(), fantoccini::error::CmdError> {
        info!("Saving page sources starting with {}", first_page_url);

        let mut page_url = Url::from_str(first_page_url)
            .expect("Failed to parse argument first_page_url into a valid URL.");

        // Search for the next page automatically, navigate to it and save source to disk:
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

    pub async fn extract_ac_product(&mut self) -> std::io::Result<()> {
        // check if path is dir and exists on disk:
        if self.page_sources_output_path.is_dir() {
            for entry in std::fs::read_dir(self.page_sources_output_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    info!("Extracting ACProduct from file {:?}", path);

                    let document = Document::from_read(File::open(path)?)?;

                    let product_items_predicate = predicate::Class("products")
                        .and(predicate::Class("list"))
                        .and(predicate::Class("items"))
                        .and(predicate::Class("product-items"))
                        .descendant(predicate::Name("li"));

                    for product_item in document.find(product_items_predicate) {
                        let product_img_predicate = predicate::Descendant(
                            predicate::Name("span").and(predicate::Class("product-image-wrapper")),
                            predicate::Name("img").and(predicate::Class("product-image-photo")),
                        );

                        for product_img_node in product_item.find(product_img_predicate) {
                            let product_img_url = product_img_node.attr("data-amsrc").unwrap_or("");

                            info!("Found product img: {:?}", product_img_url);
                        }
                    }
                }
            }
        } else {
            error!(
                "{:?} is not a directory or does not exist on disk.",
                self.page_sources_output_path
            );
        }

        Ok(())
    }

    /// Terminate WebDriver session.
    ///
    /// Calling this is necessary with some Web Drivers that don't support session sharing.
    /// Such sessions become unusable when trying to reconnect.
    pub async fn close_session(&mut self) -> Result<(), fantoccini::error::CmdError> {
        self.client.close().await
    }
}
