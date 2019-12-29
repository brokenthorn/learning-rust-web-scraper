//! A module for different scrapers.

use url::{Origin, Url};

pub mod climatico;

/// Turns a URL into a valid HTML file name that includes as much information about the original URL
/// as possible.
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
    let path = url.path().replace("/", "_slash_");
    let query_params = match url.query() {
        None => None,
        Some(q) => Some(q.replace("=", "_eq_").replace("&", "_")),
    };

    match query_params {
        None => Ok(format!("{}__{}__{}_{}.html", scheme, host, port, path)),
        Some(qparms) => Ok(format!(
            "{}__{}__{}_{}__{}.html",
            scheme, host, port, path, qparms
        )),
    }
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
    pub struct ACProduct {
        /// Product name.
        pub name: String,
        /// Manufacturer name.
        pub manufacturer: String,

        /// Uniquely identifying product code.
        pub product_code: String,
        // URL for the dedicated product page (details page).
        pub product_url: String,

        /// URL for the product page on the reseller's website.
        pub reseller_product_page_url: String,
        /// URL for the official manufacturer's product page.
        pub manufacturer_product_page_url: String,

        /// File path for main image used to list the product.
        pub listing_image_path: String,
        /// URL for the main image used to list the product.
        pub listing_image_url: String,

        pub price: f32,
        pub currency: Currency,

        /// Does the AC product have WiFi connectivity?
        pub has_wifi_connection: bool,
        /// Compatible mains voltage(s).
        pub mains_voltage: String,
        /// Internal cooling/heating unit length. Main dimension used to determine if the unit
        /// fits a certain mounting place.
        pub internal_unit_length: String,

        pub heating_noise_level: String,
        pub cooling_noise_level: String,

        pub heating_energy_class: String,
        pub cooling_energy_class: String,

        pub heating_btu_capacity: String,
        pub cooling_btu_capacity: String,

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
