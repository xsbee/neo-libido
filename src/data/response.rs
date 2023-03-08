use serde::Deserialize;

use super::{
    hit::Hit,
    utils::hits_deserialize,
};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Response {
    pub page: usize,
    pub nbPages: usize,
    pub nbHits: usize,
    pub hitsPerPage: usize,
    #[serde(deserialize_with = "hits_deserialize")]
    pub hits: Vec<Hit>,
}
