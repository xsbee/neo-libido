use serde::Deserialize;

use super::utils::stred_int_to_float;

#[derive(Deserialize, Debug)]
pub struct Stream {
    /// Id of the stream.
    pub id: u64,
    /// Same slug as in `hits'.
    pub slug: String,
    pub kind: String,
    /// Extension of the filename.
    pub extension: String,
    /// MIME type of the original file.
    pub mime_type: String,
    /// Horizontal resolution of MPEG-DASH stream.
    pub width: u16,
    /// Vertical resoution of MPEG-DASH stream.
    #[serde(deserialize_with = "stred_int_to_float")]
    pub height: u16,
    /// Duration of the file in miliseconds.
    pub duration_in_ms: u64,
    /// Size of the file in megabytes.
    pub filesize_mbs: usize,
    /// Name of the file (including extension).
    pub filename: String,
    /// URL to M3U8 (of all fragments).
    pub url: String,
    pub is_guest_allowed: bool,
    pub is_member_allowed: bool,
    pub is_premium_allowed: bool,
}