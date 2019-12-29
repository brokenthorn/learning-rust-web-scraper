//! A module for scraping `https://www.climatico.ro/`.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;

use fantoccini::{Client, Locator};
use log::{debug, error, info};
use select::document::Document;
use select::predicate;
use select::predicate::Predicate;
use url::{ParseError, Url};

use crate::scrapers::data::{ACProduct, Currency};
use crate::scrapers::url_to_html_file_name;

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

        let client = match futures::executor::block_on(Client::new("http://localhost:4444")) {
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

        let client = match futures::executor::block_on(Client::new("http://localhost:4444")) {
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

    /// Save page sources for an entire product listing, starting at [start_page_url].
    /// Automatically finds the next page and stops when it doesn't find any more pages.
    pub async fn save_page_sources(
        &mut self,
        start_page_url: &str,
    ) -> Result<(), fantoccini::error::CmdError> {
        info!(
            "Starting to save Climatico page sources starting with this page: {}",
            start_page_url
        );

        let mut page_url = Url::from_str(start_page_url)
            .expect("Failed to parse argument first_page_url into a valid URL.");

        loop {
            // try to convert the page URL to a file name + path, to save the page source as:
            let source_file_pathbuf = match url_to_html_file_name(&page_url) {
                Ok(file) => Some(self.page_sources_output_path.join(file)),
                Err(err) => {
                    error!(
                        "Could not determine file name to save page {:?}: {}",
                        page_url, err
                    );
                    None
                }
            };

            // if the URL could be converted to a file path, continue:
            if let Some(file_path) = source_file_pathbuf {
                debug!("Navigating to Climatico page {:?}", page_url);
                self.client.goto(page_url.as_ref()).await?;

                info!("Saving Climatico page {:?} to {:?}", page_url, file_path);
                let source = self.client.source().await?;
                let mut source_file = std::fs::File::create(file_path.as_path()).expect(
                    format!(
                        "Failed to create file {:?} to save page {:?}.",
                        file_path, page_url
                    )
                    .as_str(),
                );
                source_file
                    .write_all(source.as_ref())
                    .expect("Failed to write to disk.");
            }

            // else update page_url or break this loop if there are no more pages left:
            match self
                .client
                .find(Locator::Css("head > link[rel=next]"))
                .await
            {
                Ok(mut link) => {
                    let next_page_href = link
                        .attr("href")
                        .await?
                        .expect("link tag with rel=next should have an href attribute!");
                    page_url = match Url::from_str(next_page_href.as_str()) {
                        Ok(url) => url,
                        Err(_err) => {
                            error!(
                                "Failed while parsing URL for next page link: {}. Saving more Climatico page sources aborted.",
                                next_page_href
                            );
                            break;
                        }
                    }
                }
                Err(_) => {
                    info!("No more pages left.");
                    break;
                }
            }
        }

        Ok(())
    }

    // TODO: Simplify this function. It does too much.
    pub async fn extract_ac_product(&mut self) -> std::io::Result<()> {
        // check if path is dir and exists on disk:
        if self.page_sources_output_path.is_dir() {
            for entry in std::fs::read_dir(self.page_sources_output_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    info!("Extracting ACProduct from file {:?}", path);

                    let document = Document::from_read(File::open(path)?)?;

                    let product_items_predicate = predicate::Name("div")
                        .and(predicate::Attr("id", "amasty-shopby-product-list"))
                        .descendant(
                            predicate::Name("div")
                                .and(predicate::Class("products"))
                                .and(predicate::Class("wrapper"))
                                .and(predicate::Class("list"))
                                .and(predicate::Class("products-list")),
                        )
                        .descendant(
                            predicate::Name("ol")
                                .and(predicate::Class("products"))
                                .and(predicate::Class("list"))
                                .and(predicate::Class("items"))
                                .and(predicate::Class("product-items")),
                        )
                        .descendant(predicate::Name("li"));

                    for product in document.find(product_items_predicate) {
                        info!("Found ACProduct.");

                        let mut ac_product = ACProduct {
                            name: "".to_string(),
                            manufacturer: "".to_string(),
                            product_code: "".to_string(),
                            product_url: "".to_string(),
                            reseller_product_page_url: "".to_string(),
                            manufacturer_product_page_url: "".to_string(),
                            listing_image_path: "".to_string(),
                            listing_image_url: "".to_string(),
                            price: 0.0,
                            currency: Currency::RON,
                            has_wifi_connection: false,
                            mains_voltage: "".to_string(),
                            internal_unit_length: "".to_string(),
                            heating_noise_level: "".to_string(),
                            cooling_noise_level: "".to_string(),
                            heating_energy_class: "".to_string(),
                            cooling_energy_class: "".to_string(),
                            heating_btu_capacity: "".to_string(),
                            cooling_btu_capacity: "".to_string(),
                            category_drill_down: vec![],
                        };

                        let img_option = product
                            .find(
                                predicate::Name("img").and(predicate::Class("product-image-photo")),
                            )
                            .take(1)
                            .next();

                        if let Some(img) = img_option {
                            if let Some(a) = img.attr("data-amsrc") {
                                ac_product.listing_image_url = String::from(a);
                            }
                            if let Some(a) = img.attr("alt") {
                                ac_product.name = String::from(a);
                            }
                        }

                        let product_item_link_option = product
                            .find(
                                predicate::Name("strong")
                                    .and(predicate::Class("product"))
                                    .and(predicate::Class("name"))
                                    .and(predicate::Class("product"))
                                    .and(predicate::Class("product-item-name"))
                                    .and(predicate::Class("product-name"))
                                    .descendant(
                                        predicate::Name("a")
                                            .and(predicate::Class("product-item-link")),
                                    ),
                            )
                            .take(1)
                            .next();

                        if let Some(product_item_link) = product_item_link_option {
                            if let Some(a) = product_item_link.attr("href") {
                                ac_product.product_url = String::from(a);
                            }
                        }

                        let product_features_table_body_option = product
                            .find(
                                predicate::Name("table")
                                    .and(predicate::Class("prod-list-features"))
                                    .descendant(predicate::Name("tbody")),
                            )
                            .take(1)
                            .next();

                        if let Some(table_body) = product_features_table_body_option {
                            for tr in table_body.find(predicate::Name("tr")).into_iter() {
                                let label_node_option = tr.first_child();
                                let value_node_option = tr.last_child();

                                let label = label_node_option
                                    .map_or(String::from(""), |label_node| label_node.text());

                                let value = value_node_option
                                    .map_or(String::from(""), |value_node| value_node.text());

                                // info!("Found ACProduct attribute: \"{}\" = \"{}\"", label, value);

                                match label.as_str() {
                                    "Cod produs:" => ac_product.product_code = value,
                                    "Capacitate racire:" => ac_product.cooling_btu_capacity = value,
                                    "Capacitate incalzire:" => {
                                        ac_product.heating_btu_capacity = value
                                    }
                                    "Clasa energetica racire:" => {
                                        ac_product.cooling_energy_class = value
                                    }
                                    "Clasa energetica incalzire:" => {
                                        ac_product.heating_energy_class = value
                                    }
                                    "Tensiune alimentare:" => ac_product.mains_voltage = value,
                                    "Nivel de zgomot racire:" => {
                                        ac_product.cooling_noise_level = value
                                    }
                                    "Nivel de zgomot incalzire:" => {
                                        ac_product.heating_noise_level = value
                                    }
                                    "Lungime unitate interna:" => {
                                        ac_product.internal_unit_length = value
                                    }
                                    "Conexiune Wi-Fi:" => {
                                        ac_product.has_wifi_connection =
                                            if value.starts_with("D") { true } else { false }
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            info!("No product features table body found!");
                        }

                        debug!("Found AC Product: {:#?}", ac_product);
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
