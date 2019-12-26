//! A module for different scrapers.

use url::{Origin, Url};

pub mod climatico;

/// Turns a URL to a HTML page into a valid file name.
pub fn url_to_html_file_name(url: &Url) -> Result<String, String> {
    if url.cannot_be_a_base() {
        return Err("Cannot parse this URL. It cannot be a base URL.".to_string());
    }

    let origin = url.origin();
    let (scheme, host, port) = match origin {
        Origin::Opaque(_) => {
            return Err(
                "Cannot parse this URL into scheme, host and port. The origin is opaque."
                    .to_string(),
            );
        }
        Origin::Tuple(s, h, p) => (s, h.to_string(), p),
    };
    let path = url.path().replace("/", "_");
    let query_params = match url.query() {
        None => "".to_string(),
        Some(q) => q.replace("=", "_"),
    };

    Ok(format!(
        "{}__{}__{}__{}__{}.html",
        scheme, host, port, path, query_params
    ))
}

pub mod data {
    //! Common data structures used by scrapers.
    //!
    //! All data structures are serializable using the [serde] crate.

    use serde::{Deserialize, Serialize};

    /// Currency sign.
    #[derive(Debug, Serialize, Deserialize)]
    pub enum Currency {
        RON,
        USD,
        EUR,
    }

    /// AC (air conditioning) product.
    #[derive(Debug, Serialize, Deserialize)]
    pub struct ACProduct<'a> {
        /// Product name.
        pub name: &'a str,
        /// Manufacturer name.
        pub manufacturer: &'a str,

        /// Uniquely identifying product code.
        pub product_code: &'a str,

        /// URL for the product page on the reseller's website.
        pub reseller_product_page_url: &'a str,
        /// URL for the official manufacturer's product page.
        pub manufacturer_product_page_url: &'a str,

        /// File path for main image used to list the product.
        pub listing_image_path: &'a str,
        /// URL for the main image used to list the product.
        pub listing_image_url: &'a str,

        pub price: f32,
        pub currency: Currency,

        /// Does the AC product have WiFi connectivity?
        pub has_wifi_connection: bool,
        /// Compatible mains voltage(s).
        pub mains_voltage: &'a str,
        /// Internal cooling/heating unit length. Main dimension used to determine if the unit
        /// fits a certain mounting place.
        pub internal_unit_length: &'a str,

        pub heating_noise_level: &'a str,
        pub cooling_noise_level: &'a str,

        pub heating_energy_class: &'a str,
        pub cooling_energy_class: &'a str,

        pub heating_btu_capacity: &'a str,
        pub cooling_btu_capacity: &'a str,

        /// A drill down of product categories and subcategories.
        ///
        /// The drill down looks like this:
        /// * `Category → Subcategory 1 → Subcategory 2 → ...`
        ///
        /// # Examples
        ///
        /// * `["Residential", "AC", "Ceiling/Floor"]`,
        /// * `["Residential", "AC", "Split system"]`,
        /// * `["Residential", "AC", "Cassette"]`,
        /// * `["Residential", "AC", "Console"]`.
        pub category_drill_down: Vec<String>,
    }
}
