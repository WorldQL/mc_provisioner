use bytes::Bytes;
use color_eyre::Result;

mod http;
mod paper;
mod pufferfish;

pub use paper::PaperJarProvider;
pub use pufferfish::PufferfishJarProvider;

pub trait ServerJarProvider {
    fn download_jar(version: &str) -> Result<Bytes>;
}
