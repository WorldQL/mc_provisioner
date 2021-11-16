use std::path::PathBuf;

use color_eyre::Result;
use fs_extra::dir::{self, CopyOptions};
use tracing::info;

use crate::utils;

pub fn sync_plugins(server_count: u8, start_port: u16, directory_template: String) -> Result<()> {
    let plugins_dir = PathBuf::from("plugins");
    let plugins_exists = plugins_dir.as_path().exists();
    let options = {
        let mut options = CopyOptions::new();
        options.overwrite = true;

        options
    };

    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, _, directory, _) in server_iter {
        if !directory.exists() {
            continue;
        }

        if plugins_exists {
            info!("syncing plugins to server: {:?}", &directory);
            dir::copy(&plugins_dir, directory, &options)?;
        }
    }

    Ok(())
}
