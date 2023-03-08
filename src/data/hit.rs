use std::time::Duration;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Hit {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub cover_url: String,
    pub poster_url: String,
    pub brand: String,
    pub brand_id: u64,
    pub duration_in_ms: Duration,
    pub is_censored: bool,
    pub views: u64,
    pub likes: u64,
    pub dislikes: u64,
    pub downloads: u64,
    pub monthly_rank: u64,
    #[serde(rename = "created_at")]
    pub uploaded_at: u64,
    pub released_at: u64,
    pub titles: Vec<String>,
    pub tags: Vec<String>,
}