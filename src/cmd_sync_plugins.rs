use std::path::PathBuf;

use color_eyre::Result;
use fs_extra::dir::{self, CopyOptions};
use tracing::info;

use crate::config::GlobalArgs;
use crate::utils;

pub fn sync_plugins(global_args: GlobalArgs) -> Result<()> {
    let plugins_dir = PathBuf::from("plugins");
    let plugins_exists = plugins_dir.as_path().exists();
    let options = {
        let mut options = CopyOptions::new();
        options.overwrite = true;

        options
    };

    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

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
