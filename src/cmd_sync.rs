use std::fs;

use color_eyre::Result;
use fs_extra::dir::{self, CopyOptions};
use tracing::{info, warn};

use crate::config::GlobalArgs;
use crate::utils;

pub fn sync(global_args: GlobalArgs, clear_plugins: bool) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    let options = {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = false;
        options.content_only = true;

        options
    };

    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        // Clear plugins dir
        if clear_plugins {
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

        // Sync dirs
        for source_dir in &global_args.sync_dirs {
            if !source_dir.exists() {
                warn!("directory {:?} does not exist, skipping sync", source_dir);
                continue;
            }

            if !source_dir.is_dir() {
                warn!("{:?} is not a directory, skipping sync", source_dir);
                continue;
            }

            let is_plugins_dir = match source_dir.file_name() {
                Some(dir) => dir == "plugins",
                _ => false,
            };

            let target_dir = match is_plugins_dir {
                true => directory.join("plugins"),
                false => directory.clone(),
            };

            dir::copy(&source_dir, &target_dir, &options)?;
        }
    }

    Ok(())
}
