use std::collections::HashMap;

use bytes::Bytes;
use color_eyre::Result;
use serde::Deserialize;
use tracing::info;

use super::http::CLIENT;
use super::ServerJarProvider;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
#[allow(dead_code)]
struct PaperBuildResponse {
    project_id: String,
    project_name: String,
    version: String,
    build: u16,
    downloads: HashMap<String, PaperBuildDownload>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

pub struct PaperJarProvider;
impl ServerJarProvider for PaperJarProvider {
    fn download_jar(version: &str) -> Result<Bytes> {
        let build_id = latest_paper_build(version)?;
        let download_url = paper_url(version, build_id)?;

        info!("downloading paper.jar build {} for {}", build_id, version);
        let bytes = CLIENT.get(download_url).send()?.bytes()?;

        Ok(bytes)
    }
}
