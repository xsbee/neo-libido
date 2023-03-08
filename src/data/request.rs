use serde::Serialize;

use super::utils::{ordering_serialize, tags_mode_serialize};

/// Possible criteria to sort hits with.
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

/// Hentai search request. Contains various options for tuning search.
/// 
/// ```
/// let req = SearchRequest {
///     query: "kanojo",
/// 
///     ..Default::default()
/// };
/// ```
#[derive(Serialize)]
pub struct SearchRequest<'b, 'a: 'b> {
    /// List of case-insensitive blacklist of tags.
    /// 
    /// A hit will be filtered out if it matches any of the tags listed here.
    #[serde(rename = "blacklist")]
    pub tags_blacklist: &'b [&'a str],
    /// List of case-insensitive list of brands.
    /// 
    /// A hit whose brand-name matches any of the brands listed here would be returned.
    pub brands: &'b [&'a str],
    /// List of tags.
    /// 
    /// Matching mode is specified by [`tags_exclusive`](SearchRequest.tags_exclusive).
    pub tags: &'b [&'a str],
    /// Crietion to sort hits with.
    pub order_by: Ordering,
    /// If to display results in ascending order with respect to their property
    /// in [`order_by`](SearchRequest.order_by).
    #[serde(rename = "ordering", serialize_with = "ordering_serialize")]
    pub ascending: bool,
    /// If to match when hit has one of [`tags`](SearchRequest.tags)' tag,
    /// or to match when all tags in [`tags`](SearchRequest.tags) are present in hit.
    /// 
    /// In mathematical terms: if to match when hit's tags and `tags`' set intersection
    /// is non-empty, or to match if `tags`' is a subset of hit's tags.
    #[serde(rename = "tags_mode", serialize_with = "tags_mode_serialize")]
    pub tags_exclusive: bool,
    /// Page number of results.
    /// 
    /// Data is paginated in this API. Simply, result list is divided into pages.
    pub page: usize,
    /// Case-insensitive title (name) of hentai to search for.
    /// 
    /// This is a non-fuzzy search. Rigoriously, this string must be a substring
    /// of the title.
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