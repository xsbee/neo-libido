use serde::Deserialize;

use super::utils::stred_int_to_float;

#[derive(Deserialize, Debug)]
pub struct Stream {
    pub id: u64,
    pub slug: String,
    pub kind: String,
    pub extension: String,
    pub mime_type: String,
    pub width: u16,
    #[serde(deserialize_with = "stred_int_to_float")]
    pub height: u16,
    pub duration_in_ms: u64,
    pub filesize_mbs: usize,
    pub filename: String,
    pub url: String,
    pub is_guest_allowed: bool,
    pub is_member_allowed: bool,
    pub is_premium_allowed: bool,
}