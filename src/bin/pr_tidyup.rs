use std::env;

use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use log::{info, warn};
use octocrab::{params, OctocrabBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    Builder::from_env(Env::default().default_filter_or("info")).init();
    // FIXME: only gets the first 100
    const MAX_PRS: u8 = 100;
    const OWNER: &str = "rust-unofficial";
    const REPO: &str = "awesome-rust";
    let instance = OctocrabBuilder::new()
        .personal_token(env::var("GITHUB_TOKEN").context("Getting GITHUB_TOKEN")?)
        .build()?;
    let pulls = instance.pulls(OWNER, REPO);
    let issues = instance.issues(OWNER, REPO);
    let prs = pulls
        .list()
        .state(params::State::Open)
        .per_page(MAX_PRS)
        .page(0u32)
        .send()
        .await?;
    if prs.items.len() == MAX_PRS as usize {
        warn!("PR count has exceeded {MAX_PRS}, need looping code");
    }
    info!("{} open PRs", prs.items.len());
    for pr in prs {
        if pr.title.contains("RustChain") {
            info!("PR {} is a RustChain PR, closing", pr.number);
            issues
                .create_comment(pr.number, "From the tidyup bot: this PR is being automatically closed due to the spam from RustChain. Please do not open more PRs for this, they will be closed as well.")
                .await
                .context(format!("While commenting on {}", pr.number))?;
            pulls
                .update(pr.number)
                .state(params::pulls::State::Closed)
                .send()
                .await
                .context(format!("While closing {}", pr.number))?;
        }
    }

    Ok(())
}
