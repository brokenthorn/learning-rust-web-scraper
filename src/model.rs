use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ShopifyProduct {
    #[serde(rename = "Handle")]
    pub handle: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Body (HTML)")]
    pub body_html: Option<String>,
    #[serde(rename = "Vendor")]
    pub vendor: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    #[serde(rename = "Tags")]
    pub tags: Option<String>,
    #[serde(rename = "Published")]
    pub published: Option<String>,
    #[serde(rename = "Option1 Name")]
    pub option1_name: Option<String>,
    #[serde(rename = "Option1 Value")]
    pub option1_value: Option<String>,
    #[serde(rename = "Option2 Name")]
    pub option2_name: Option<String>,
    #[serde(rename = "Option2 Value")]
    pub option2_value: Option<String>,
    #[serde(rename = "Option3 Name")]
    pub option3_name: Option<String>,
    #[serde(rename = "Option3 Value")]
    pub option3_value: Option<String>,
    #[serde(rename = "Variant SKU")]
    pub variant_sku: Option<String>,
    #[serde(rename = "Variant Grams")]
    pub variant_grams: Option<String>,
    #[serde(rename = "Variant Inventory Tracker")]
    pub variant_inventory_tracker: Option<String>,
    #[serde(rename = "Variant Inventory Qty")]
    pub variant_inventory_qty: Option<String>,
    #[serde(rename = "Variant Inventory Policy")]
    pub variant_inventory_policy: Option<String>,
    #[serde(rename = "Variant Fulfillment Service")]
    pub variant_fulfillment_service: Option<String>,
    #[serde(rename = "Variant Price")]
    pub variant_price: Option<String>,
    #[serde(rename = "Variant Compare At Price")]
    pub variant_compare_at_price: Option<String>,
    #[serde(rename = "Variant Requires Shipping")]
    pub variant_requires_shipping: Option<String>,
    #[serde(rename = "Variant Taxable")]
    pub variant_taxable: Option<String>,
    #[serde(rename = "Variant Barcode")]
    pub variant_barcode: Option<String>,
    #[serde(rename = "Image Src")]
    pub image_src: Option<String>,
    #[serde(rename = "Image Position")]
    pub image_position: Option<String>,
    #[serde(rename = "Image Alt Text")]
    pub image_alt_text: Option<String>,
    #[serde(rename = "Gift Card")]
    pub gift_card: Option<String>,
    #[serde(rename = "SEO Title")]
    pub seo_title: Option<String>,
    #[serde(rename = "SEO Description")]
    pub seo_description: Option<String>,
    #[serde(rename = "Google Shopping / Google Product Category")]
    pub google_shopping_google_product_category: Option<String>,
    #[serde(rename = "Google Shopping / Gender")]
    pub google_shopping_gender: Option<String>,
    #[serde(rename = "Google Shopping / Age Group")]
    pub google_shopping_age_group: Option<String>,
    #[serde(rename = "Google Shopping / MPN")]
    pub google_shopping_mpn: Option<String>,
    #[serde(rename = "Google Shopping / AdWords Grouping")]
    pub google_shopping_ad_words_grouping: Option<String>,
    #[serde(rename = "Google Shopping / AdWords Labels")]
    pub google_shopping_ad_words_labels: Option<String>,
    #[serde(rename = "Google Shopping / Condition")]
    pub google_shopping_condition: Option<String>,
    #[serde(rename = "Google Shopping / Custom Product")]
    pub google_shopping_custom_product: Option<String>,
    #[serde(rename = "Google Shopping / Custom Label 0")]
    pub google_shopping_custom_label_0: Option<String>,
    #[serde(rename = "Google Shopping / Custom Label 1")]
    pub google_shopping_custom_label_1: Option<String>,
    #[serde(rename = "Google Shopping / Custom Label 2")]
    pub google_shopping_custom_label_2: Option<String>,
    #[serde(rename = "Google Shopping / Custom Label 3")]
    pub google_shopping_custom_label_3: Option<String>,
    #[serde(rename = "Google Shopping / Custom Label 4")]
    pub google_shopping_custom_label_4: Option<String>,
    #[serde(rename = "Variant Image")]
    pub variant_image: Option<String>,
    #[serde(rename = "Variant Weight Unit")]
    pub variant_weight_unit: Option<String>,
    #[serde(rename = "Variant Tax Code")]
    pub variant_tax_code: Option<String>,
    #[serde(rename = "Cost per item")]
    pub cost_per_item: Option<String>,
}

impl Default for ShopifyProduct {
    fn default() -> Self {
        ShopifyProduct {
            handle: None,
            title: None,
            body_html: None,
            vendor: None,
            r#type: None,
            tags: None,
            published: None,
            option1_name: None,
            option1_value: None,
            option2_name: None,
            option2_value: None,
            option3_name: None,
            option3_value: None,
            variant_sku: None,
            variant_grams: None,
            variant_inventory_tracker: None,
            variant_inventory_qty: None,
            variant_inventory_policy: None,
            variant_fulfillment_service: None,
            variant_price: None,
            variant_compare_at_price: None,
            variant_requires_shipping: None,
            variant_taxable: None,
            variant_barcode: None,
            image_src: None,
            image_position: None,
            image_alt_text: None,
            gift_card: None,
            seo_title: None,
            seo_description: None,
            google_shopping_google_product_category: None,
            google_shopping_gender: None,
            google_shopping_age_group: None,
            google_shopping_mpn: None,
            google_shopping_ad_words_grouping: None,
            google_shopping_ad_words_labels: None,
            google_shopping_condition: None,
            google_shopping_custom_product: None,
            google_shopping_custom_label_0: None,
            google_shopping_custom_label_1: None,
            google_shopping_custom_label_2: None,
            google_shopping_custom_label_3: None,
            google_shopping_custom_label_4: None,
            variant_image: None,
            variant_weight_unit: None,
            variant_tax_code: None,
            cost_per_item: None,
        }
    }
}
