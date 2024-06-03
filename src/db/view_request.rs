use serde::{Deserialize, Serialize};

use crate::{player::animation::Animation, tree::tree::Tree};

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewAnimationRequest {
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

impl ViewAnimationRequest {
    pub fn from(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ViewTreeRequest {
    #[serde(rename = "page")]
    pub page: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "totalItems")]
    pub total_items: u32,
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
    #[serde(rename = "items")]
    pub items: Vec<Tree>,
}

impl ViewTreeRequest {
    pub fn from(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

