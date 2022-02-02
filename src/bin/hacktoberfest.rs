// Helper tool to dump all repos in awesome-rust that are tagged with "hacktoberfest"

use chrono::{DateTime, Duration, Local};
use failure::{format_err, Error, Fail};
use futures::future::{select_all, BoxFuture, FutureExt};
use lazy_static::lazy_static;
use log::{debug, warn};
use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;
use reqwest::redirect::Policy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io::Write;
use std::time;
use std::u8;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;

#[derive(Debug, Fail, Serialize, Deserialize)]
enum CheckerError {
    #[fail(display = "http error: {}", status)]
    HttpError {
        status: u16,
        location: Option<String>,
    },
}

struct MaxHandles {
    remaining: Semaphore,
}

struct Handle<'a> {
    _permit: SemaphorePermit<'a>,
}

impl MaxHandles {
    fn new(max: usize) -> MaxHandles {
        MaxHandles {
            remaining: Semaphore::new(max),
        }
    }

    async fn get<'a>(&'a self) -> Handle<'a> {
        let permit = self.remaining.acquire().await.unwrap();
        return Handle { _permit: permit };
    }
}

impl<'a> Drop for Handle<'a> {
    fn drop(&mut self) {
        debug!("Dropping");
    }
}

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .danger_accept_invalid_certs(true) // because some certs are out of date
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:68.0) Gecko/20100101 Firefox/68.0") // so some sites (e.g. sciter.com) don't reject us
        .redirect(Policy::none())
        .pool_max_idle_per_host(0)
        .timeout(time::Duration::from_secs(20))
        .build().unwrap();

    // This is to avoid errors with running out of file handles, so we only do 20 requests at a time
    static ref HANDLES: MaxHandles = MaxHandles::new(20);
}

lazy_static! {
    static ref GITHUB_REPO_REGEX: Regex =
        Regex::new(r"^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/?$").unwrap();
    static ref GITHUB_API_REGEX: Regex = Regex::new(r"https://api.github.com/").unwrap();
}

#[derive(Deserialize, Debug)]
struct RepoInfo {
    full_name: String,
    description: Option<String>,
    topics: Vec<String>,
}

async fn get_hacktoberfest_core(github_url: String) -> Result<Info, CheckerError> {
    warn!("Downloading Hacktoberfest label for {}", github_url);
    let rewritten = GITHUB_REPO_REGEX
        .replace_all(&github_url, "https://api.github.com/repos/$org/$repo")
        .to_string();
    let mut req = CLIENT.get(&rewritten);
    if let Ok(username) = env::var("GITHUB_USERNAME") {
        if let Ok(password) = env::var("GITHUB_TOKEN") {
            // needs a token with at least public_repo scope
            req = req.basic_auth(username, Some(password));
        }
    }

    let resp = req.send().await;
    match resp {
        Err(err) => {
            warn!("Error while getting {}: {}", github_url, err);
            return Err(CheckerError::HttpError {
                status: err.status().unwrap().as_u16(),
                location: Some(github_url.to_string()),
            });
        }
        Ok(ok) => {
            if !ok.status().is_success() {
                return Err(CheckerError::HttpError {
                    status: ok.status().as_u16(),
                    location: None,
                });
            }
            let raw = ok.text().await.unwrap();
            match serde_json::from_str::<RepoInfo>(&raw) {
                Ok(val) => Ok(Info {
                    name: val.full_name,
                    description: val.description.unwrap_or_default(),
                    hacktoberfest: val.topics.iter().find(|t| *t == "hacktoberfest").is_some(),
                }),
                Err(_) => {
                    panic!("{}", raw);
                }
            }
        }
    }
}

fn get_hacktoberfest(url: String) -> BoxFuture<'static, (String, Result<Info, CheckerError>)> {
    debug!("Need handle for {}", url);
    async move {
        let _handle = HANDLES.get().await;
        return (url.clone(), get_hacktoberfest_core(url).await);
    }
    .boxed()
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    hacktoberfest: bool,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    updated_at: DateTime<Local>,
    info: Info,
}

type Results = BTreeMap<String, Link>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut results: Results = fs::read_to_string("results/hacktoberfest.yaml")
        .map_err(|e| format_err!("{}", e))
        .and_then(|x| serde_yaml::from_str(&x).map_err(|e| format_err!("{}", e)))
        .unwrap_or(Results::new());

    let mut url_checks = vec![];

    let mut do_check = |url: String| {
        if !url.starts_with("http") {
            return;
        }
        if used.contains(&url) {
            return;
        }
        used.insert(url.clone());
        if let Some(_) = results.get(&url) {
            return;
        }
        let check = get_hacktoberfest(url).boxed();
        url_checks.push(check);
    };

    let mut to_check: Vec<String> = vec![];

    for (event, _) in parser.into_offset_iter() {
        match event {
            Event::Start(tag) => match tag {
                Tag::Link(_link_type, url, _title) | Tag::Image(_link_type, url, _title) => {
                    if GITHUB_REPO_REGEX.is_match(&url) {
                        to_check.push(url.to_string());
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    for url in to_check {
        do_check(url)
    }

    let results_keys = results.keys().cloned().collect::<BTreeSet<String>>();
    let old_links = results_keys.difference(&used);
    for link in old_links {
        results.remove(link).unwrap();
    }
    fs::write("results/results.yaml", serde_yaml::to_string(&results)?)?;

    let mut not_written = 0;
    let mut last_written = Local::now();

    let mut failed: u32 = 0;
    while url_checks.len() > 0 {
        debug!("Waiting for {}", url_checks.len());
        let ((url, res), _index, remaining) = select_all(url_checks).await;
        url_checks = remaining;
        match res {
            Ok(info) => {
                print!("\u{2714} ");
                if let Some(link) = results.get_mut(&url) {
                    link.updated_at = Local::now();
                    link.info = info
                } else {
                    results.insert(
                        url.clone(),
                        Link {
                            updated_at: Local::now(),
                            info: info,
                        },
                    );
                }
            }
            Err(_) => {
                print!("\u{2718} ");
                println!("{}", url);
                failed += 1;
            }
        }
        std::io::stdout().flush().unwrap();

        not_written += 1;
        let duration = Local::now() - last_written;
        if duration > Duration::seconds(5) || not_written > 20 {
            fs::write(
                "results/hacktoberfest.yaml",
                serde_yaml::to_string(&results)?,
            )?;
            not_written = 0;
            last_written = Local::now();
        }
    }
    fs::write(
        "results/hacktoberfest.yaml",
        serde_yaml::to_string(&results)?,
    )?;
    println!("");

    if failed == 0 {
        println!("All awesome-rust repos tagged with 'hacktoberfest'");
        let mut sorted_repos = results
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        sorted_repos.sort_by_key(|a| a.to_lowercase());
        for name in sorted_repos {
            let link = results.get(&name).unwrap();
            if link.info.hacktoberfest {
                println!(
                    "* [{}]({}) - {}",
                    link.info.name, name, link.info.description
                )
            }
        }
        Ok(())
    } else {
        Err(format_err!("{} urls with errors", failed))
    }
}
