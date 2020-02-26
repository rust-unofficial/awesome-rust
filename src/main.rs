use pulldown_cmark::{Parser, Event, Tag};
use std::fs;
use futures::future::{select_all, BoxFuture, FutureExt};
use std::collections::{BTreeSet, BTreeMap};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU32, Ordering};
use async_std::task;
use std::time;
use log::{warn, debug, info};
use std::io::Write;
use reqwest::{Client, redirect::Policy, StatusCode, header, Url};
use regex::Regex;
use scraper::{Html, Selector};
use failure::{Fail, Error, format_err};

#[derive(Debug, Fail)]
enum CheckerError {
    #[fail(display = "failed to try url")]
    NotTried, // Generally shouldn't happen, but useful to have

    #[fail(display = "http error: {}", status)]
    HttpError {
        status: StatusCode,
        location: Option<String>,
    },

    #[fail(display = "reqwest error: {}", error)]
    ReqwestError {
        error: reqwest::Error,
    },

    #[fail(display = "travis build is unknown")]
    TravisBuildUnknown,

    #[fail(display = "travis build image with no branch")]
    TravisBuildNoBranch,

    #[fail(display = "github actions image with no branch")]
    GithubActionNoBranch,
}

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
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:68.0) Gecko/20100101 Firefox/68.0") // so some sites (e.g. sciter.com) don't reject us
        .redirect(Policy::none())
        .timeout(time::Duration::from_secs(20))
        .build().unwrap();

    // This is to avoid errors with running out of file handles, so we only do 20 requests at a time
    static ref HANDLES: MaxHandles = MaxHandles::new(20);
}

fn get_url(url: String) -> BoxFuture<'static, (String, Result<(), CheckerError>)> {
    async move {
        let _handle = HANDLES.get().await;
        let mut res = Err(CheckerError::NotTried);
        for _ in 0..5u8 {
            debug!("Running {}", url);
            let resp = CLIENT
                .get(&url)
                .header(header::ACCEPT, "image/svg+xml, text/html, */*;q=0.8")
                .send()
                .await;
            match resp {
                Err(err) => {
                    warn!("Error while getting {}, retrying: {}", url, err);
                    res = Err(CheckerError::ReqwestError{error: err});
                    continue;
                }
                Ok(ok) => {
                    let status = ok.status();
                    if status != StatusCode::OK {
                        lazy_static! {
                            static ref ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/actions(?:\?workflow=.+)?").unwrap();
                            static ref YOUTUBE_REGEX: Regex = Regex::new(r"https://www.youtube.com/watch\?v=(?P<video_id>.+)").unwrap();
                            static ref AZURE_BUILD_REGEX: Regex = Regex::new(r"https://dev.azure.com/[^/]+/[^/]+/_build").unwrap();
                        }
                        if status == StatusCode::NOT_FOUND && ACTIONS_REGEX.is_match(&url) {
                            let rewritten = ACTIONS_REGEX.replace_all(&url, "https://github.com/$org/$repo");
                            warn!("Got 404 with Github actions, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url(rewritten.to_string()).await;
                            return (url, res);
                        }
                        if status == StatusCode::FOUND && YOUTUBE_REGEX.is_match(&url) {
                            // Based off of https://gist.github.com/tonY1883/a3b85925081688de569b779b4657439b
                            // Guesswork is that the img feed will cause less 302's than the main url
                            // See https://github.com/rust-unofficial/awesome-rust/issues/814 for original issue
                            let rewritten = YOUTUBE_REGEX.replace_all(&url, "http://img.youtube.com/vi/$video_id/mqdefault.jpg");
                            warn!("Got 302 with Youtube, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url(rewritten.to_string()).await;
                            return (url, res);
                        };
                        if status == StatusCode::FOUND && AZURE_BUILD_REGEX.is_match(&url) {
                            // Azure build urls always redirect to a particular build id, so no stable url guarantees
                            let redirect = ok.headers().get(header::LOCATION).unwrap().to_str().unwrap();
                            let merged_url = Url::parse(&url).unwrap().join(redirect).unwrap();
                            info!("Got 302 from Azure devops, so replacing {} with {}", url, merged_url);
                            let (_new_url, res) = get_url(merged_url.into_string()).await;
                            return (url, res);
                        }

                        warn!("Error while getting {}, retrying: {}", url, status);
                        if status.is_redirection() {
                            res = Err(CheckerError::HttpError {status: status, location: ok.headers().get(header::LOCATION).and_then(|h| h.to_str().ok()).map(|x| x.to_string())});
                            break;
                        } else {
                            res = Err(CheckerError::HttpError {status: status, location: None});
                            continue;
                        }
                    }
                    lazy_static! {
                        static ref TRAVIS_IMG_REGEX: Regex = Regex::new(r"https://api.travis-ci.(?:com|org)/[^/]+/.+\.svg(\?.+)?").unwrap();
                        static ref GITHUB_ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/[^/]+/[^/]+/workflows/[^/]+/badge.svg(\?.+)?").unwrap();
                    }
                    if let Some(matches) = TRAVIS_IMG_REGEX.captures(&url) {
                        // Previously we checked the Content-Disposition headers, but sometimes that is incorrect
                        // We're now looking for the explicit text "unknown" in the middle of the SVG
                        let content = ok.text().await.unwrap();
                        if content.contains("unknown") {
                            res = Err(CheckerError::TravisBuildUnknown);
                            break;
                        }
                        let query = matches.get(1).map(|x| x.as_str()).unwrap_or("");
                        if !query.starts_with("?") || query.find("branch=").is_none() {
                            res = Err(CheckerError::TravisBuildNoBranch);
                            break;
                        }
                    }
                    if let Some(matches) = GITHUB_ACTIONS_REGEX.captures(&url) {
                        debug!("Github actions match {:?}", matches);
                        let query = matches.get(1).map(|x| x.as_str()).unwrap_or("");
                        if !query.starts_with("?") || query.find("branch=").is_none() {
                            res = Err(CheckerError::GithubActionNoBranch);
                            break;
                        }
                    }
                    debug!("Finished {}", url);
                    res = Ok(());
                    break;
                }
            }
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
async fn main() -> Result<(), Error> {
    env_logger::init();
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut results: Results = fs::read_to_string("results.yaml")
        .map_err(|e| format_err!("{}", e))
        .and_then(|x| serde_yaml::from_str(&x).map_err(|e| format_err!("{}", e)))
        .unwrap_or(Results::new());
    results.failed.clear();

    let mut url_checks = vec![];

    let mut do_check = |url: String| {
        if !url.starts_with("http") {
            return;
        }
        if results.working.contains(&url) {
            return;
        }
        let check = get_url(url).boxed();
        url_checks.push(check);
    };

    for (event, _range) in parser.into_offset_iter() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Link(_link_type, url, _title) | Tag::Image(_link_type, url, _title) => {
                        do_check(url.to_string());
                    }
                    _ => {}
                }
            }
            Event::Html(content) => {
                let fragment = Html::parse_fragment(&content);
                for element in fragment.select(&Selector::parse("img").unwrap()) {
                    let img_src = element.value().attr("src");
                    if let Some(src) = img_src {
                        do_check(src.to_string());
                    }
                }
                for element in fragment.select(&Selector::parse("a").unwrap()) {
                    let a_href = element.value().attr("href");
                    if let Some(href) = a_href {
                        do_check(href.to_string());
                    }
                }
            }
            _ => {}
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
                let message = match err {
                    CheckerError::HttpError {status, location} => {
                        match location {
                            Some(loc) => {
                                format!("[{}] {} -> {}", status.as_u16(), url, loc)
                            }
                            None => {
                                format!("[{}] {}", status.as_u16(), url)
                            }
                        }
                    }
                    CheckerError::TravisBuildUnknown => {
                        format!("[Unknown travis build] {}", url)
                    }
                    CheckerError::TravisBuildNoBranch => {
                        format!("[Travis build image with no branch specified] {}", url)
                    }
                    CheckerError::GithubActionNoBranch => {
                        format!("[Github action image with no branch specified] {}", url)
                    }
                    _ => {
                        format!("{:?}", err)
                    }
                };
                results.failed.insert(url, message);
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
        for (_url, error) in &results.failed {
            println!("{}", error);
        }
        Err(format_err!("{} urls with errors", results.failed.len()))
    }
}