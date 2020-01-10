use pulldown_cmark::{Parser, Event, Tag};
use std::fs;
use futures::future::select_all;
use futures::future::FutureExt;
use std::collections::{BTreeSet, BTreeMap};
use serde::{Serialize, Deserialize};
use anyhow::Result;

fn to_anyhow<T, E>(res: std::result::Result<T, E>) -> Result<T>
    where E: std::error::Error + std::marker::Send + std::marker::Sync + 'static
{
    res.map_err(|x| Into::<anyhow::Error>::into(x))
}

async fn get_url(url: String) -> (String, Result<String>) {
    let res = reqwest::get(&url).await;
    (url, to_anyhow(res.map(|x| format!("{:?}", x))))
}

#[derive(Debug, Serialize, Deserialize)]
struct Results {
    working: BTreeSet<String>,
    failed: BTreeMap<String, String>
}

impl Results {
    fn new() -> Results {
        Results {
            working: BTreeSet::new(),
            failed: BTreeMap::new()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut results: Results = to_anyhow(fs::read_to_string("results.yaml")).and_then(|x| to_anyhow(serde_yaml::from_str(&x))).unwrap_or(Results::new());
    results.failed.clear();

    let mut url_checks = vec![];

    for (event, _range) in parser.into_offset_iter() {
        if let Event::Start(tag) = event {
            if let Tag::Link(_link_type, url, _title) = tag {
                if !url.starts_with("http") {
                    continue;
                }
                let url_string = url.to_string();
                if results.working.contains(&url_string) {
                    continue;
                }
                let check = get_url(url_string).boxed();
                url_checks.push(check);
            }
        }
    }

    while url_checks.len() > 0 {
        let ((url, res), _index, remaining) = select_all(url_checks).await;
        url_checks = remaining;
        match res {
            Ok(_) => {
                print!("\u{2714} ");
                results.working.insert(url);
            },
            Err(err) => {
                print!("\u{2718} ");
                results.failed.insert(url, err.to_string());
            }
        }
        fs::write("results.yaml", serde_yaml::to_string(&results)?)?;
    }
    println!("");
    for (url, error) in &results.failed {
        println!("Error: {} {}", url, error);
    }
    Ok(())
}