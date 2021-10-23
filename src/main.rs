use std::{collections::HashMap, convert::Infallible};

use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let api = warp::get()
        .and(warp::path("api"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| p.get("url").unwrap().to_string())
        .and_then(json_builder);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

async fn json_builder(url: String) -> Result<impl Reply, Infallible> {
    let resp = reqwest::get(url).await.unwrap();
    let text = resp.text().await.unwrap();
    Ok(format!("{}", text))
}
