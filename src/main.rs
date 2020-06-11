extern crate alfred;
extern crate clap;
extern crate curl;

use clap::{App, Arg};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    stargazers_count: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Items {
    items: Vec<Repo>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query").short("q").required(true).index(1))
        .get_matches();

    let query = args.value_of("query").unwrap();

    let url = format!("https://api.github.com/search/repositories?q={}", query);

    let client = reqwest::Client::new();
    let resp = client
        .get(url.as_str())
        .header(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_static("reqwest"),
        )
        .send()
        .await
        .unwrap()
        .json::<Items>()
        .await
        .unwrap()
        .items;

    let items = resp
        .into_iter()
        .map(|item| {
            alfred::ItemBuilder::new(item.full_name.clone())
                .arg(item.id.to_string())
                .quicklook_url(item.html_url.clone())
                .text_large_type(item.full_name)
                .subtitle(item.description.unwrap_or(String::from("")))
                .variable("URL", item.html_url)
                .into_item()
        })
        .collect::<Vec<alfred::Item>>();

    alfred::json::Builder::with_items(&items)
        .write(std::io::stdout())
        .expect("Couldn't write items to Alfred");
    Ok(())
}
