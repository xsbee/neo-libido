use std::io::{stdout, Write};
use reqwest;

mod data;

fn main() -> reqwest::Result<()> {
    let mut stdout = stdout().lock();

    let client = reqwest::blocking::Client::new();
    let query = data::request::SearchRequest {
        tags_blacklist: &[],
        brands: &[],
        tags: &[],
        order_by: data::request::Ordering::ReleaseDate,
        ascending: false,
        tags_exclusive: false,
        page: 1,
        query: "",
    };

    let hits = client
        .post("https://search.htv-services.com/")
        .json(&query)
        .send()?
        .json::<data::response::Response>()?
        .hits;

    let hits_servers = hits.iter().map(|hit| {
        (
            hit,
            client
                .get(format!("https://hanime.tv/api/v8/video?id={}", hit.slug))
                .send()
                .and_then(|r| r.json::<data::video::Video>())
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

    Ok(())
}
