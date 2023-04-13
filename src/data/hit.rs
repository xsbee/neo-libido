use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Hit {
    /// ID of this hit.
    pub id: u64,
    /// Title (in Romaji) of Hentai
    pub name: String,
    /// String (kebab-case) to refer this entry.
    ///
    /// Slug is used in a similiar way as YouTube video IDs.
    pub slug: String,
    /// (HTML formatted) Description of Hentai.
    pub description: String,
    /// URL to cover art.
    ///
    /// This is the image seen on DVD or Blu-Ray for that hentai.
    /// More specifically, as seen when searching on _hanime.tv_.
    pub cover_url: String,
    /// URL to poster art.
    ///
    /// This is a preview of the actual Hentai video, seen in the recommended feed.
    pub poster_url: String,
    /// Publisher (not necessarily producer) of the Hentai.
    pub brand: String,
    /// ID of the brand.
    pub brand_id: u64,
    /// Duration of Hentai.
    ///
    /// Hentai (or Anime in general) usually does not exceed more than 30 minutes.
    /// For anime it is due to aligning to televsion time slot (including ads).
    pub duration_in_ms: u64,
    /// If uncensored version of Hentai is not available.
    ///
    /// https://en.wikipedia.org/wiki/Pornography_in_Japan#Censorship_laws
    pub is_censored: bool,
    /// Amount of views (not restricted to signed up members) the Hentai video has.
    pub views: u64,
    /// Amount of likes the Hentai video has.
    pub likes: u64,
    /// Amount of dislikes the Hentai video has.
    pub dislikes: u64,
    /// Number of times, it has been downloaded (the official way).
    pub downloads: u64,
    /// Monthly rank of this hit.
    pub monthly_rank: u64,
    /// Time when it was uploaded.
    #[serde(rename = "created_at")]
    pub uploaded_at: u64,
    /// Time when it was released.
    pub released_at: u64,
    /// List of alternative titles (if it has any).
    pub titles: Vec<String>,
    /// List of tags.
    pub tags: Vec<String>,
}
