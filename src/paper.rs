use std::collections::HashMap;

use bytes::Bytes;
use color_eyre::Result;
use once_cell::sync::Lazy;
use reqwest::blocking::{Client, ClientBuilder};
use serde::Deserialize;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
});

#[derive(Debug, Deserialize)]
struct PaperVersionResponse {
    project_id: String,
    project_name: String,
    version: String,
    builds: Vec<u16>,
}

fn latest_paper_build(version: &str) -> Result<u16> {
    let url = format!(
        "https://papermc.io/api/v2/projects/paper/versions/{}",
        version
    );

    let response = CLIENT.get(url).send()?.json::<PaperVersionResponse>()?;
    let latest_build = response.builds.into_iter().max().unwrap();

    Ok(latest_build)
}

#[derive(Debug, Deserialize)]
struct PaperBuildResponse {
    project_id: String,
    project_name: String,
    version: String,
    build: u16,
    downloads: HashMap<String, PaperBuildDownload>,
}

#[derive(Debug, Deserialize)]
struct PaperBuildDownload {
    name: String,
    sha256: String,
}

fn paper_url(version: &str, build_id: u16) -> Result<String> {
    let url = format!(
        "https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}",
        version, build_id
    );

    let response = CLIENT.get(url).send()?.json::<PaperBuildResponse>()?;
    let file_name = &response.downloads.get("application").unwrap().name;

    let url = format!(
        "https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}/downloads/{}",
        version, build_id, file_name
    );

    Ok(url)
}

pub fn download_paper(version: &str) -> Result<Bytes> {
    let build_id = latest_paper_build(version)?;
    let download_url = paper_url(version, build_id)?;

    let bytes = CLIENT.get(download_url).send()?.bytes()?;
    Ok(bytes)
}
