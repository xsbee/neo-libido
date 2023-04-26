use std::io::{prelude::*, stdout};

use clap::Parser;

mod data;

use data::{
    request::{Ordering, SearchRequest},
    response::{Response, self},
    video::Video
};

#[derive(Parser)]
struct Cli {
    /// Search string (within titles)
    query: Option<String>,
    #[arg(short, long, default_value_t = 1)]
    /// Page # of results
    page: usize,
    /// If to download all search results
    #[arg(short = 'a', long, default_value_t = false)]
    download_all: bool,
    /// If to download all in a certain franchise
    #[arg(short, long, default_value_t = false)]
    franchise_all: bool,
    /// List of tags to look for
    #[arg(short, long)]
    tags: Vec<String>,
    /// List of brands to look for
    #[arg(short, long)]
    brands: Vec<String>,
    /// Crietion to sorting results
    #[arg(short, long, value_enum, default_value_t = Ordering::UploadDate)]
    order_by: Ordering,

    /// List of tags to not look for
    #[arg(long)]
    tags_blacklist: Vec<String>,
    /// If to show in ascending order
    #[arg(long, default_value_t = false)]
    ascending: bool,
    /// Match with any tag in `--tags'
    #[arg(long, default_value_t = false)]
    broad_search: bool,
}

use reqwest;

// TODO put this structure elsewhere
pub(crate) struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub(crate) fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub(crate) fn search(&self, req: &SearchRequest) -> PageIterator {
        PageIterator { req: req.clone(), client: &self.client, num_pages: None }
    }

    pub(crate) fn fetch_video(&self, slug: &str) -> reqwest::Result<Video> {
        self.client
            .get(format!("https://hanime.tv/api/v8/video?id={}", slug))
            .send()?
            .json::<Video>()
    }
}

struct PageIterator<'b> {
    req: SearchRequest,
    client: &'b reqwest::blocking::Client,
    num_pages: Option<usize>
}

// TODO Implement absolute indexing, facilitating use of .nth()
impl<'b> Iterator for PageIterator<'b> {
    type Item = reqwest::Result<response::Response>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.num_pages {
            if self.req.page > n {
                return None;
            }
        }

        let response = self.client
            .post("https://search.htv-services.com/")
            .json(&self.req)
            .send()
            .and_then(|x| x.json::<Response>());

        return match response {
            Ok(r) => {
                if r.nbPages == 0 {
                    return None;
                }

                self.num_pages = Some(r.nbPages);
                self.req.page += 1;
            
                Some(Ok(r))
            },
            Err(e) => {
                Some(Err(e))    
            }
        }
    }
}

fn main() -> reqwest::Result<()> {
    let mut stdout = stdout().lock();
    let cli = Cli::parse();

    let client = Client::new();
    let query = SearchRequest {
        tags_blacklist: cli.tags_blacklist,
        brands: cli.brands,
        tags: cli.tags,
        order_by: cli.order_by,
        ascending: cli.ascending,
        tags_exclusive: !cli.broad_search,
        page: cli.page,
        query: cli.query.unwrap_or("".to_string()),
    };

    client.search(&query);

    for page in client.search(&query) {
        let page = page?;
        let hits_servers = page.hits.iter().map(|hit| {
            (
                hit,
                client
                    .fetch_video(&hit.slug)
                    .map(|r| r.videos_manifest.servers),
            )
        });

        for (hit, servers) in hits_servers {
            if let Ok(servers) = servers {
                if let Some(stream) = servers
                    .iter()
                    .flat_map(|v| &v.streams)
                    .filter(|s| s.url.len() != 0)
                    .max_by_key(|y| y.height)
                {
                    writeln!(stdout, "{} {}", hit.name, stream.url).unwrap();
                } else {
                    eprintln!("No stream found for \"{}\"", hit.name);
                }
            } else if let Err(e) = servers {
                eprintln!(
                    "An error occured while fetching video manifest for \"{}\": {:#?}",
                    hit.name, e
                );
            }
        }
    }

    Ok(())
}
