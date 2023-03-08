use serde::Deserialize;

use super::stream::Stream;

#[derive(Deserialize, Debug)]
pub struct Server {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub is_permanent: bool,
    pub streams: Vec<Stream>,
}