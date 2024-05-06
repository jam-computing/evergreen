use crate::player::animation::Animation;
use serde::{Deserialize, Serialize};

use super::db_item::DbItem;

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewRequest<T> where T: DbItem {
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "items")]
    pub items: Vec<Animation>,
}

impl<T> ViewRequest<T>  where T: DbItem{
     fn from(json: &str) -> serde_json::Result<T> {
        let view: ViewRequest<T> = serde_json::from_str(json)?;
        Ok(view)
    }
}
