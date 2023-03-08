use serde::Deserialize;

use super::server::Server;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub servers: Vec<Server>,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    pub videos_manifest: Manifest,
}