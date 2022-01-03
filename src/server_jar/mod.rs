use bytes::Bytes;
use color_eyre::Result;

use self::paper::PaperJarProvider;

mod http;
mod paper;

pub(crate) trait ServerJarProvider {
    fn download_jar(version: &str) -> Result<Bytes>;
}

pub fn download_jar(version: &str) -> Result<Bytes> {
    // TODO: Switch based on input
    PaperJarProvider::download_jar(version)
}
