use crate::player::animation::Animation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewRequest {
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

impl ViewRequest {
    pub fn from(json: &str) -> serde_json::Result<Self> {
        let view: ViewRequest = serde_json::from_str(json)?;
        Ok(view)
    }
}
