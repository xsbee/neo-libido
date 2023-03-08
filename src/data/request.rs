use serde::Serialize;

use super::utils::{ordering_serialize, tags_mode_serialize};

#[derive(Serialize, Default)]
pub enum Ordering {
    #[serde(rename = "title_sortable")]
    Title,
    #[serde(rename = "views")]
    Views,
    #[serde(rename = "likes")]
    Likes,
    #[serde(rename = "created_at_unix")]
    UploadDate,
    #[serde(rename = "released_at_unix")]
    #[default]
    ReleaseDate,
}

#[derive(Serialize)]
pub struct SearchRequest<'b, 'a: 'b> {
    #[serde(rename = "blacklist")]
    pub tags_blacklist: &'b [&'a str],
    pub brands: &'b [&'a str],
    pub tags: &'b [&'a str],
    pub order_by: Ordering,
    #[serde(rename = "ordering", serialize_with = "ordering_serialize")]
    pub ascending: bool,
    #[serde(rename = "tags_mode", serialize_with = "tags_mode_serialize")]
    pub tags_exclusive: bool,
    pub page: usize,
    #[serde(rename = "search_text")]
    pub query: &'a str,
}

impl<'b, 'a> Default for SearchRequest<'b, 'a> {
    fn default() -> Self {
        Self {
            tags_blacklist: &["ugly bastard", "futnari"],
            brands: &[],
            tags: &[],
            order_by: Ordering::default(),
            ascending: false,
            tags_exclusive: false,
            page: 1,
            query: ""
        }
    }
}