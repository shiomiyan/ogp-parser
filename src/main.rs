use select::{
    document::Document,
    predicate::{And, Attr, Class, Element, Name, Or, Predicate},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible};
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let api = warp::get()
        .and(warp::path("api"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| p.get("url").unwrap().to_string())
        .and_then(fetch_meta);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct MetaInfo {
    og_title: String,
    og_image: String,
    og_description: String,
    og_type: String,
    og_url: String,
    og_sitename: String,
}

#[derive(Debug)]
struct Pair {
    property: String,
    content: String,
}

async fn fetch_meta(url: String) -> Result<impl Reply, Infallible> {
    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();
    let document = Document::from_read(body.as_bytes()).unwrap();
    let result = document
        .find(Name("meta"))
        .filter(|v| v.attrs().collect::<Vec<_>>().len() == 2)
        .filter_map(|v| {
            let attrs = v.attrs().collect::<Vec<_>>();
            if attrs[0].0 == "property" && attrs[1].0 == "content" {
                Some(Pair {
                    property: attrs[0].1.to_string(),
                    content: attrs[1].1.to_string(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(format!("{:?}", result))
}
