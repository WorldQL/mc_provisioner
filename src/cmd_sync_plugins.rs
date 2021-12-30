use std::fs;
use std::path::PathBuf;

use color_eyre::Result;
use fs_extra::dir::{self, CopyOptions};
use tracing::{info, warn};

use crate::config::GlobalArgs;
use crate::utils;

pub fn sync_plugins(global_args: GlobalArgs, clear: bool) -> Result<()> {
    let plugins_dir = PathBuf::from("plugins");
    let plugins_exists = plugins_dir.as_path().exists();

    if !plugins_exists && !clear {
        warn!("plugins directory does not exist, skipping...");
        return Ok(());
    } else if !plugins_exists && clear {
        warn!("plugins directory does not exist, no syncing will occur");
    }

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
        let name = directory.to_str().unwrap();
        if !directory.exists() {
            warn!("directory for server {} does not exist, skipping...", &name);
            continue;
        }

        if clear {
            let server_plugins_dir = directory.join("plugins");
            if !server_plugins_dir.exists() {
                continue;
            }

            info!("clearing plugins dir in server: {}", &name);
            for entry in fs::read_dir(server_plugins_dir)? {
                let entry = entry?;
                let file_type = entry.file_type()?;

                if !file_type.is_file() {
                    continue;
                }

                let path = entry.path();
                let extension = match path.extension() {
                    None => continue,
                    Some(extension) => extension.to_string_lossy().to_lowercase(),
                };

                if extension == "jar" {
                    fs::remove_file(&path)?;
                }
            }
        }

        if plugins_exists {
            info!("syncing plugins to server: {}", &name);
            dir::copy(&plugins_dir, directory, &options)?;
        }
    }

    Ok(())
}
