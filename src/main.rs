use chrono::{DateTime, Duration, Local};
use diffy::create_patch;
use failure::{format_err, Error, Fail};
use futures::future::{select_all, BoxFuture, FutureExt};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use pulldown_cmark::{Event, Parser, Tag};
use regex::Regex;
use reqwest::{header, redirect::Policy, Client, StatusCode, Url};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::io::Write;
use std::time;
use std::u8;
use std::{cmp::Ordering, fs};
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;

const MINIMUM_GITHUB_STARS: u32 = 50;
const MINIMUM_CARGO_DOWNLOADS: u32 = 2000;

// Allow overriding the needed stars for a section. "level" is the header level in the markdown, default is MINIMUM_GITHUB_STARS
// In general, we should just use the defaults. However, for some areas where there's not a lot of well-starred projects, but a
// a few that are say just below the thresholds, then it's worth reducing the thresholds so we can get a few more projects.
fn override_stars(level: u32, text: &str) -> Option<u32> {
    if level == 2 && text.contains("Resources") {
        // This is zero because a lot of the resources are non-github/non-cargo links and overriding for all would be annoying
        // These should be evaluated with more primitive means
        Some(0)
    } else if level == 3 && text.contains("Games") {
        Some(40)
    } else if level == 3 && text.contains("Emulators") {
        Some(40)
    } else {
        None // i.e. use defaults
    }
}

lazy_static! {
    // Overrides for popularity count, each needs a good reason (i.e. downloads/stars we don't support automatic counting of)
    // Each is a URL that's "enough" for an item to pass the popularity checks
    static ref POPULARITY_OVERRIDES: Vec<String> = vec![
        "https://github.com/maidsafe".to_string(), // Many repos of Rust code, collectively > 50 stars
        "https://pijul.org".to_string(), // Uses it's own VCS at https://nest.pijul.com/pijul/pijul with 190 stars at last check
        "https://gitlab.com/veloren/veloren".to_string(), // No direct gitlab support, but >1000 stars there
        "https://gitlab.redox-os.org/redox-os/redox".to_string(), // 394 stars
        "https://amp.rs".to_string(), // https://github.com/jmacdonald/amp has 2.9k stars
        "https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb".to_string(), // > 350k downloads
        "https://gitpod.io".to_string(), // https://github.com/gitpod-io/gitpod has 4.7k stars
        "https://wiki.gnome.org/Apps/Builder".to_string(), // https://gitlab.gnome.org/GNOME/gnome-builder has 133 stars
        "https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml".to_string(), // > 860k downloads
        "https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer".to_string(), // > 260k downloads
        "https://marketplace.visualstudio.com/items?itemName=rust-lang.rust".to_string(), // > 1M downloads
        "https://docs.rs".to_string(), // https://github.com/rust-lang/docs.rs has >600 stars
        "https://github.com/rust-bio".to_string(), // https://github.com/rust-bio/rust-bio on it's own has >900 stars
        "https://github.com/contain-rs".to_string(), // Lots of repos with good star counts
        "https://github.com/georust".to_string(), // Lots of repos with good star counts
        "http://kiss3d.org".to_string(), // https://github.com/sebcrozet/kiss3d has >900 stars
        "https://github.com/rust-qt".to_string(), // Various high-stars repositories
        "https://chromium.googlesource.com/chromiumos/platform/crosvm/".to_string(), // Can't tell count directly, but various mirrors of it (e.g. https://github.com/dgreid/crosvm) have enough stars that it's got enough interest
        "https://seed-rs.org/".to_string(), // https://github.com/seed-rs/seed has 2.1k stars
        "https://crates.io".to_string(), // This one gets a free pass :)
        "https://cloudsmith.com/cargo-registry/".to_string() // First private cargo registry (https://cloudsmith.com/blog/worlds-first-private-cargo-registry-w-cloudsmith-rust/) and not much in the way of other options yet. See also https://github.com/rust-unofficial/awesome-rust/pull/1141#discussion_r688711555
    ];
}

#[derive(Debug, Fail, Serialize, Deserialize)]
enum CheckerError {
    #[fail(display = "failed to try url")]
    NotTried, // Generally shouldn't happen, but useful to have

    #[fail(display = "http error: {}", status)]
    HttpError {
        status: u16,
        location: Option<String>,
    },

    #[fail(display = "too many requests")]
    TooManyRequests,

    #[fail(display = "reqwest error: {}", error)]
    ReqwestError { error: String },

    #[fail(display = "travis build is unknown")]
    TravisBuildUnknown,

    #[fail(display = "travis build image with no branch")]
    TravisBuildNoBranch,
}

fn formatter(err: &CheckerError, url: &String) -> String {
    match err {
        CheckerError::HttpError { status, location } => match location {
            Some(loc) => {
                format!("[{}] {} -> {}", status, url, loc)
            }
            None => {
                format!("[{}] {}", status, url)
            }
        },
        CheckerError::TravisBuildUnknown => {
            format!("[Unknown travis build] {}", url)
        }
        CheckerError::TravisBuildNoBranch => {
            format!("[Travis build image with no branch specified] {}", url)
        }
        _ => {
            format!("{:?}", err)
        }
    }
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

fn get_url(url: String) -> BoxFuture<'static, (String, Result<(), CheckerError>)> {
    debug!("Need handle for {}", url);
    async move {
        let _handle = HANDLES.get().await;
        return get_url_core(url).await;
    }
    .boxed()
}

lazy_static! {
    static ref GITHUB_REPO_REGEX: Regex =
        Regex::new(r"^https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)(.*)").unwrap();
    static ref GITHUB_API_REGEX: Regex = Regex::new(r"https://api.github.com/").unwrap();
    static ref CRATE_REGEX: Regex =
        Regex::new(r"https://crates.io/crates/(?P<crate>[^/]+)/?$").unwrap();
}

#[derive(Deserialize, Debug)]
struct GithubStars {
    stargazers_count: u32,
}

async fn get_stars(github_url: &str) -> Option<u32> {
    warn!("Downloading Github stars for {}", github_url);
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
            return None;
        }
        Ok(ok) => {
            let raw = ok.text().await.unwrap();
            let data = match serde_json::from_str::<GithubStars>(&raw) {
                Ok(val) => val,
                Err(_) => {
                    panic!("{:?}", raw);
                }
            };
            return Some(data.stargazers_count);
        }
    }
}

#[derive(Deserialize, Debug)]
struct CrateInfo {
    downloads: u64,
}

#[derive(Deserialize, Debug)]
struct Crate {
    #[serde(rename = "crate")]
    info: CrateInfo,
}

async fn get_downloads(github_url: &str) -> Option<u64> {
    warn!("Downloading Crates downloads for {}", github_url);
    let rewritten = CRATE_REGEX
        .replace_all(&github_url, "https://crates.io/api/v1/crates/$crate")
        .to_string();
    let req = CLIENT.get(&rewritten);

    let resp = req.send().await;
    match resp {
        Err(err) => {
            warn!("Error while getting {}: {}", github_url, err);
            return None;
        }
        Ok(ok) => {
            let data = ok.json::<Crate>().await.unwrap();
            return Some(data.info.downloads);
        }
    }
}

fn get_url_core(url: String) -> BoxFuture<'static, (String, Result<(), CheckerError>)> {
    async move {
        let mut res = Err(CheckerError::NotTried);
        for _ in 0..5u8 {
            debug!("Running {}", url);
            if env::var("GITHUB_USERNAME").is_ok() && env::var("GITHUB_TOKEN").is_ok() && GITHUB_REPO_REGEX.is_match(&url) {
                let rewritten = GITHUB_REPO_REGEX.replace_all(&url, "https://api.github.com/repos/$org/$repo");
                info!("Replacing {} with {} to workaround rate limits on Github", url, rewritten);
                let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                return (url, res);
            }
            let mut req = CLIENT
                .get(&url)
                .header(header::ACCEPT, "image/svg+xml, text/html, */*;q=0.8");

            if GITHUB_API_REGEX.is_match(&url) {
                if let Ok(username) = env::var("GITHUB_USERNAME") {
                    if let Ok(password) = env::var("GITHUB_TOKEN") {
                        // needs a token with at least public_repo scope
                        info!("Using basic auth for {}", url);
                        req = req.basic_auth(username, Some(password));
                    }
                }
            }

            let resp = req.send().await;
            match resp {
                Err(err) => {
                    warn!("Error while getting {}, retrying: {}", url, err);
                    res = Err(CheckerError::ReqwestError{error: err.to_string()});
                    continue;
                }
                Ok(ok) => {
                    let status = ok.status();
                    if status != StatusCode::OK {
                        lazy_static! {
                            static ref ACTIONS_REGEX: Regex = Regex::new(r"https://github.com/(?P<org>[^/]+)/(?P<repo>[^/]+)/actions(?:\?workflow=.+)?").unwrap();
                            static ref YOUTUBE_VIDEO_REGEX: Regex = Regex::new(r"https://www.youtube.com/watch\?v=(?P<video_id>.+)").unwrap();
                            static ref YOUTUBE_PLAYLIST_REGEX: Regex = Regex::new(r"https://www.youtube.com/playlist\?list=(?P<playlist_id>.+)").unwrap();
                            static ref YOUTUBE_CONSENT_REGEX: Regex = Regex::new(r"https://consent.youtube.com/m\?continue=.+").unwrap();
                            static ref AZURE_BUILD_REGEX: Regex = Regex::new(r"https://dev.azure.com/[^/]+/[^/]+/_build").unwrap();
                        }
                        if status == StatusCode::NOT_FOUND && ACTIONS_REGEX.is_match(&url) {
                            let rewritten = ACTIONS_REGEX.replace_all(&url, "https://github.com/$org/$repo");
                            warn!("Got 404 with Github actions, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                            return (url, res);
                        }
                        if status == StatusCode::FOUND && YOUTUBE_VIDEO_REGEX.is_match(&url) {
                            // Based off of https://gist.github.com/tonY1883/a3b85925081688de569b779b4657439b
                            // Guesswork is that the img feed will cause less 302's than the main url
                            // See https://github.com/rust-unofficial/awesome-rust/issues/814 for original issue
                            let rewritten = YOUTUBE_VIDEO_REGEX.replace_all(&url, "http://img.youtube.com/vi/$video_id/mqdefault.jpg");
                            warn!("Got 302 with Youtube, so replacing {} with {}", url, rewritten);
                            let (_new_url, res) = get_url_core(rewritten.to_string()).await;
                            return (url, res);
                        };
                        if status == StatusCode::FOUND && YOUTUBE_PLAYLIST_REGEX.is_match(&url) {
                            let location = ok.headers().get("LOCATION").map(|h| h.to_str().unwrap()).unwrap_or_default();
                            if YOUTUBE_CONSENT_REGEX.is_match(location) {
                                warn!("Got Youtube consent link for {}, so assuming playlist is ok", url);
                                return (url, Ok(()));
                            }
                        };
                        if status == StatusCode::FOUND && AZURE_BUILD_REGEX.is_match(&url) {
                            // Azure build urls always redirect to a particular build id, so no stable url guarantees
                            let redirect = ok.headers().get(header::LOCATION).unwrap().to_str().unwrap();
                            let merged_url = Url::parse(&url).unwrap().join(redirect).unwrap();
                            info!("Got 302 from Azure devops, so replacing {} with {}", url, merged_url);
                            let (_new_url, res) = get_url_core(merged_url.into_string()).await;
                            return (url, res);
                        }

                        if status == StatusCode::TOO_MANY_REQUESTS {
                            // We get a lot of these, and we should not retry as they'll just fail again
                            warn!("Error while getting {}: {}", url, status);
                            return (url, Err(CheckerError::TooManyRequests));
                        }

                        warn!("Error while getting {}, retrying: {}", url, status);
                        if status.is_redirection() {
                            res = Err(CheckerError::HttpError {status: status.as_u16(), location: ok.headers().get(header::LOCATION).and_then(|h| h.to_str().ok()).map(|x| x.to_string())});
                            break;
                        } else {
                            res = Err(CheckerError::HttpError {status: status.as_u16(), location: None});
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
enum Working {
    Yes,
    No(CheckerError),
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
    last_working: Option<DateTime<Local>>,
    updated_at: DateTime<Local>,
    working: Working,
}

type Results = BTreeMap<String, Link>;

#[derive(Debug, Serialize, Deserialize)]
struct PopularityData {
    pub github_stars: BTreeMap<String, u32>,
    pub cargo_downloads: BTreeMap<String, u32>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let markdown_input = fs::read_to_string("README.md").expect("Can't read README.md");
    let parser = Parser::new(&markdown_input);

    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut results: Results = fs::read_to_string("results/results.yaml")
        .map_err(|e| format_err!("{}", e))
        .and_then(|x| serde_yaml::from_str(&x).map_err(|e| format_err!("{}", e)))
        .unwrap_or(Results::new());

    let mut popularity_data: PopularityData = fs::read_to_string("results/popularity.yaml")
        .map_err(|e| format_err!("{}", e))
        .and_then(|x| serde_yaml::from_str(&x).map_err(|e| format_err!("{}", e)))
        .unwrap_or(PopularityData {
            github_stars: BTreeMap::new(),
            cargo_downloads: BTreeMap::new(),
        });

    let mut url_checks = vec![];

    let min_between_checks: Duration = Duration::days(3);
    let max_allowed_failed: Duration = Duration::days(7);
    let mut do_check = |url: String| {
        if !url.starts_with("http") {
            return;
        }
        if used.contains(&url) {
            return;
        }
        used.insert(url.clone());
        if let Some(link) = results.get(&url) {
            if let Working::Yes = link.working {
                let since = Local::now() - link.updated_at;
                if since < min_between_checks {
                    return;
                }
            }
        }
        let check = get_url(url).boxed();
        url_checks.push(check);
    };

    let mut to_check: Vec<String> = vec![];

    #[derive(Debug)]
    struct ListInfo {
        data: Vec<String>,
    }

    let mut list_items: Vec<ListInfo> = Vec::new();
    let mut in_list_item = false;
    let mut list_item: String = String::new();

    let mut link_count: u8 = 0;
    let mut github_stars: Option<u32> = None;
    let mut cargo_downloads: Option<u32> = None;

    let mut required_stars: u32 = MINIMUM_GITHUB_STARS;
    let mut last_level: u32 = 0;
    let mut star_override_level: Option<u32> = None;

    for (event, _range) in parser.into_offset_iter() {
        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Link(_link_type, url, _title) | Tag::Image(_link_type, url, _title) => {
                        if !url.starts_with("#") {
                            let new_url = url.to_string();
                            if POPULARITY_OVERRIDES.contains(&new_url) {
                                github_stars = Some(MINIMUM_GITHUB_STARS);
                            } else if GITHUB_REPO_REGEX.is_match(&url) {
                                let github_url = GITHUB_REPO_REGEX
                                    .replace_all(&url, "https://github.com/$org/$repo")
                                    .to_string();
                                let existing = popularity_data.github_stars.get(&github_url);
                                if let Some(stars) = existing {
                                    // Use existing star data, but re-retrieve url to check aliveness
                                    // Some will have overrides, so don't check the regex yet
                                    github_stars = Some(*stars)
                                } else {
                                    github_stars = get_stars(&github_url).await;
                                    if let Some(raw_stars) = github_stars {
                                        popularity_data
                                            .github_stars
                                            .insert(github_url.to_string(), raw_stars);
                                        if raw_stars >= required_stars {
                                            fs::write(
                                                "results/popularity.yaml",
                                                serde_yaml::to_string(&popularity_data)?,
                                            )?;
                                        }
                                        link_count += 1;
                                        continue;
                                    }
                                }
                            }
                            if CRATE_REGEX.is_match(&url) {
                                let existing = popularity_data.cargo_downloads.get(&new_url);
                                if let Some(downloads) = existing {
                                    cargo_downloads = Some(*downloads);
                                } else {
                                    let raw_downloads = get_downloads(&url).await;
                                    if let Some(positive_downloads) = raw_downloads {
                                        cargo_downloads = Some(
                                            positive_downloads.clamp(0, u32::MAX as u64) as u32,
                                        );
                                        popularity_data
                                            .cargo_downloads
                                            .insert(new_url, cargo_downloads.unwrap());
                                        if cargo_downloads.unwrap_or(0) >= MINIMUM_CARGO_DOWNLOADS {
                                            fs::write(
                                                "results/popularity.yaml",
                                                serde_yaml::to_string(&popularity_data)?,
                                            )?;
                                        }
                                    }
                                    link_count += 1;
                                    continue;
                                }
                            }

                            to_check.push(url.to_string());
                            link_count += 1;
                        }
                    }
                    Tag::List(_) => {
                        if in_list_item && list_item.len() > 0 {
                            list_items.last_mut().unwrap().data.push(list_item.clone());
                            in_list_item = false;
                        }
                        list_items.push(ListInfo { data: Vec::new() });
                    }
                    Tag::Item => {
                        if in_list_item && list_item.len() > 0 {
                            list_items.last_mut().unwrap().data.push(list_item.clone());
                        }
                        in_list_item = true;
                        list_item = String::new();
                        link_count = 0;
                        github_stars = None;
                        cargo_downloads = None;
                    }
                    Tag::Heading(level) => {
                        last_level = level;
                        if let Some(override_level) = star_override_level {
                            if level == override_level {
                                star_override_level = None;
                                required_stars = MINIMUM_GITHUB_STARS;
                            }
                        }
                    }
                    Tag::Paragraph => {}
                    _ => {
                        if in_list_item {
                            in_list_item = false;
                        }
                    }
                }
            }
            Event::Text(text) => {
                let possible_override = override_stars(last_level, &text);
                if let Some(override_value) = possible_override {
                    star_override_level = Some(last_level);
                    required_stars = override_value;
                }

                if in_list_item {
                    list_item.push_str(&text);
                }
            }
            Event::End(tag) => {
                match tag {
                    Tag::Item => {
                        if list_item.len() > 0 {
                            if link_count > 0 {
                                if github_stars.unwrap_or(0) < required_stars
                                    && cargo_downloads.unwrap_or(0) < MINIMUM_CARGO_DOWNLOADS
                                {
                                    if github_stars.is_none() {
                                        warn!("No valid github link");
                                    }
                                    if cargo_downloads.is_none() {
                                        warn!("No valid crates link");
                                    }
                                    return Err(format_err!("Not high enough metrics ({:?} stars < {}, and {:?} cargo downloads < {}): {}", github_stars, required_stars, cargo_downloads, MINIMUM_CARGO_DOWNLOADS, list_item));
                                }
                            }
                            list_items.last_mut().unwrap().data.push(list_item.clone());
                            list_item = String::new();
                        }
                        in_list_item = false
                    }
                    Tag::List(_) => {
                        let list_info = list_items.pop().unwrap();
                        if list_info.data.iter().find(|s| *s == "License").is_some()
                            && list_info.data.iter().find(|s| *s == "Resources").is_some()
                        {
                            // Ignore wrong ordering in top-level list
                            continue;
                        }
                        let mut sorted_recent_list = list_info.data.to_vec();
                        sorted_recent_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                        let joined_recent = list_info.data.join("\n");
                        let joined_sorted = sorted_recent_list.join("\n");
                        let patch = create_patch(&joined_recent, &joined_sorted);
                        if patch.hunks().len() > 0 {
                            println!("{}", patch);
                            return Err(format_err!("Sorting error"));
                        }
                    }
                    _ => {}
                }
            }
            Event::Html(content) => {
                return Err(format_err!(
                    "Contains HTML content, not markdown: {}",
                    content
                ));
            }
            _ => {}
        }
    }
    fs::write(
        "results/popularity.yaml",
        serde_yaml::to_string(&popularity_data)?,
    )?;

    to_check.sort_by(|a, b| {
        let get_time = |k| {
            let res = results.get(k);
            if let Some(link) = res {
                if let Some(last_working) = link.last_working {
                    Some(last_working)
                } else {
                    None
                }
            } else {
                None
            }
        };
        let res_a = get_time(a);
        let res_b = get_time(b);
        if res_a.is_none() {
            if res_b.is_none() {
                return a.cmp(b);
            } else {
                Ordering::Less
            }
        } else if res_b.is_none() {
            Ordering::Greater
        } else {
            res_a.unwrap().cmp(&res_b.unwrap())
        }
    });

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
    while url_checks.len() > 0 {
        debug!("Waiting for {}", url_checks.len());
        let ((url, res), _index, remaining) = select_all(url_checks).await;
        url_checks = remaining;
        match res {
            Ok(_) => {
                print!("\u{2714} ");
                if let Some(link) = results.get_mut(&url) {
                    link.updated_at = Local::now();
                    link.last_working = Some(Local::now());
                    link.working = Working::Yes;
                } else {
                    results.insert(
                        url.clone(),
                        Link {
                            updated_at: Local::now(),
                            last_working: Some(Local::now()),
                            working: Working::Yes,
                        },
                    );
                }
            }
            Err(err) => {
                print!("\u{2718} ");
                if let Some(link) = results.get_mut(&url) {
                    link.updated_at = Local::now();
                    link.working = Working::No(err);
                } else {
                    results.insert(
                        url.clone(),
                        Link {
                            updated_at: Local::now(),
                            working: Working::No(err),
                            last_working: None,
                        },
                    );
                }
            }
        }
        std::io::stdout().flush().unwrap();

        not_written += 1;
        let duration = Local::now() - last_written;
        if duration > Duration::seconds(5) || not_written > 20 {
            fs::write("results/results.yaml", serde_yaml::to_string(&results)?)?;
            not_written = 0;
            last_written = Local::now();
        }
    }
    fs::write("results/results.yaml", serde_yaml::to_string(&results)?)?;
    println!("");
    let mut failed: u32 = 0;

    for (url, link) in results.iter() {
        if let Working::No(ref err) = link.working {
            match err {
                CheckerError::HttpError { status, .. }
                    if *status == 301 || *status == 302 || *status == 404 =>
                {
                    println!("{} {:?}", url, link);
                    failed += 1;
                    continue;
                }
                CheckerError::TooManyRequests => {
                    // too many tries
                    if link.last_working.is_some() {
                        info!(
                            "Ignoring 429 failure on {} as we've seen success before",
                            url
                        );
                        continue;
                    }
                }
                _ => {}
            };
            if let Some(last_working) = link.last_working {
                let since = Local::now() - last_working;
                if since > max_allowed_failed {
                    println!("{} {:?}", url, link);
                    failed += 1;
                } else {
                    println!(
                        "Failure occurred but only {}, so we're not worrying yet: {}",
                        chrono_humanize::HumanTime::from(-since),
                        formatter(err, url)
                    );
                }
            } else {
                println!("{} {:?}", url, link);
                failed += 1;
                continue;
            }
        }
    }
    if failed == 0 {
        println!("No errors!");
        Ok(())
    } else {
        Err(format_err!("{} urls with errors", failed))
    }
}
