use bytes::Bytes;
use color_eyre::Result;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

use crate::server_jar::{PaperJarProvider, PufferfishJarProvider, ServerJarProvider};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum JarType {
    Paper,
    Pufferfish,
}

impl Default for JarType {
    fn default() -> Self {
        Self::Paper
    }
}

impl JarType {
    pub fn file_name(&self) -> String {
        format!("{}.jar", self)
    }

    pub fn download(&self, version: &str) -> Result<Bytes> {
        match self {
            JarType::Paper => PaperJarProvider::download_jar(version),
            JarType::Pufferfish => PufferfishJarProvider::download_jar(version),
        }
    }
}
