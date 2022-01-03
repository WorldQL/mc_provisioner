use bytes::Bytes;
use color_eyre::Result;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::{error, info};

use super::http::CLIENT;
use super::ServerJarProvider;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PufferfishArtifactResponse {
    number: u32,
    artifacts: Vec<PufferfishArtifact>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
struct PufferfishArtifact {
    display_path: String,
    file_name: String,
    relative_path: String,
}

fn get_latest_artifact_url(version: &str) -> Result<(u32, String)> {
    let url = format!(
        "https://ci.pufferfish.host/job/Pufferfish-{}/lastSuccessfulBuild/api/json",
        version
    );

    let response = CLIENT.get(url).send()?;

    if response.status() == StatusCode::NOT_FOUND {
        error!("unsupported pufferfish version: {}", version);
        std::process::exit(1)
    }

    let response = response
        .error_for_status()?
        .json::<PufferfishArtifactResponse>()?;

    let jar_artifact = match response.artifacts.get(0) {
        None => {
            error!("no pufferfish build artifacts found");
            std::process::exit(1)
        }

        Some(artifact) => artifact,
    };

    let artifact_url = format!(
        "https://ci.pufferfish.host/job/Pufferfish-{}/lastSuccessfulBuild/artifact/{}",
        version, jar_artifact.relative_path
    );

    Ok((response.number, artifact_url))
}

pub struct PufferfishJarProvider;
impl ServerJarProvider for PufferfishJarProvider {
    fn download_jar(version: &str) -> Result<Bytes> {
        let (build_id, artifact_url) = get_latest_artifact_url(version)?;

        info!(
            "downloading pufferfish.jar build {} for {}",
            build_id, version
        );

        let bytes = CLIENT.get(artifact_url).send()?.bytes()?;
        Ok(bytes)
    }
}
