use pulldown_cmark::{Parser, Event, Tag};
use std::fs;
use futures::future::{select_all, BoxFuture, FutureExt};
use std::collections::{BTreeSet, BTreeMap};
use serde::{Serialize, Deserialize};
use anyhow::Result;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU32, Ordering};
use async_std::task;
use std::time;
use log::{warn, debug};
use std::io::Write;
use reqwest::{Client, redirect::Policy, StatusCode, header};
use regex::Regex;

struct MaxHandles {
    remaining: AtomicU32
}

struct Handle<'a> {
    parent: &'a MaxHandles
}

impl MaxHandles {
    fn new(max: u32) -> MaxHandles {
        MaxHandles { remaining: AtomicU32::new(max) }
    }

    async fn get<'a>(&'a self) -> Handle<'a> {
        loop {
            let current = self.remaining.load(Ordering::Relaxed);
            if current > 0 {
                let new_current = self.remaining.compare_and_swap(current, current - 1, Ordering::Relaxed);
                if new_current == current { // worked
                    debug!("Got handle with {}", new_current);
                    return Handle { parent: self };
                }
            }
            task::sleep(time::Duration::from_millis(500)).await;
        }
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        debug!("Dropping");
        self.parent.remaining.fetch_add(1, Ordering::Relaxed);
    }
}

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .danger_accept_invalid_certs(true) // because some certs are out of date
        .user_agent("curl/7.54.0") // so some sites (e.g. sciter.com) don't reject us
        .redirect(Policy::none())
        .timeout(time::Duration::from_secs(20))
        .build().unwrap();

    // This is to avoid errors with running out of file handles, so we only do 20 requests at a time
    static ref HANDLES: MaxHandles = MaxHandles::new(20);
}

fn to_anyhow<T, E>(res: std::result::Result<T, E>) -> Result<T>
    where E: std::error::Error + std::marker::Send + std::marker::Sync + 'static
{
    res.map_err(|x| Into::<anyhow::Error>::into(x))
}

fn get_url(url: String) -> BoxFuture<'static, (String, Result<String>)> {
    async move {
        let _handle = HANDLES.get().await;
        let mut res = Err(anyhow::anyhow!("Should always try at least once.."));
        for _ in 0..5u8 {
            debug!("Running {}", url);
            let resp = CLIENT
                .get(&url)
                .header(header::ACCEPT, "text/html, */*;q=0.8")
                .send()
                .await;
            match resp {
                Err(err) => {
                    warn!("Error while getting {}, retrying: {}", url, err);
                    continue;
                }
                Ok(ref ok) => {
                    let status = ok.status();
                    if status != StatusCode::OK {
                        lazy_static! {
                            static ref ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/actions(?:\?workflow=.+)?").unwrap();
                        }
                        if status == StatusCode::NOT_FOUND && ACTIONS_REGEX.is_match(&url) {
                            let rewritten = ACTIONS_REGEX.replace_all(&url, "https://github.com/$org/$repo");
                            warn!("Got 404 with Github actions, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url(rewritten.to_string()).await;
                            return (url, res);
                        }

                        warn!("Error while getting {}, retrying: {}", url, status);
                        if status.is_redirection() {
                            res = Err(anyhow::anyhow!("Got status code {} redirecting to {}", status, ok.headers().get(header::LOCATION).and_then(|h| h.to_str().ok()).unwrap_or("<unknown>")));
                        } else {
                            res = Err(anyhow::anyhow!("Got status code {}", status));
                        }
                        continue;
                    }
                }
            }
            debug!("Finished {}", url);
            res = to_anyhow(resp.map(|x| format!("{:?}", x)));
            break;
        }
        (url, res)
    }.boxed()
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
    env_logger::init();
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut results: Results = to_anyhow(fs::read_to_string("results.yaml")).and_then(|x| to_anyhow(serde_yaml::from_str(&x))).unwrap_or(Results::new());
    results.failed.clear();

    let mut url_checks = vec![];

    for (event, _range) in parser.into_offset_iter() {
        if let Event::Start(tag) = event {
            match tag {
                Tag::Link(_link_type, url, _title) | Tag::Image(_link_type, url, _title) => {
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
                _ => {}
            }
        }
    }

    while url_checks.len() > 0 {
        debug!("Waiting...");
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
        std::io::stdout().flush().unwrap();
        fs::write("results.yaml", serde_yaml::to_string(&results)?)?;
    }
    println!("");
    if results.failed.is_empty() {
        println!("No errors!");
        Ok(())
    } else {
        for (url, error) in &results.failed {
            println!("Error: {} {}", url, error);
        }
        Err(anyhow::anyhow!("{} urls with errors", results.failed.len()))
    }
}