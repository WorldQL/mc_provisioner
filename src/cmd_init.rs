use std::fs;

use color_eyre::Result;
use fs_extra::dir::{self, CopyOptions};
use tracing::{info, warn};

use crate::config::{GlobalArgs, InitArgs};
use crate::utils;

pub fn init(global_args: GlobalArgs, args: InitArgs) -> Result<()> {
    let server_jar = global_args.jar_type.download(&global_args.jar_version)?;

    let has_ops = !args.ops.is_empty();
    let ops = args
        .ops
        .into_iter()
        .map(|p| format!("{}\n", p))
        .collect::<String>();

    let has_white_list = !args.white_list.is_empty();
    let white_list = args
        .white_list
        .into_iter()
        .map(|p| format!("{}\n", p))
        .collect::<String>();

    let extra_props = args
        .server_properties
        .into_iter()
        .map(|p| format!("{}\n", p))
        .collect::<String>();

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

    for (_, port, directory, motd) in server_iter {
        info!("creating server: {:?}", &directory);
        if !directory.exists() {
            fs::create_dir(&directory)?;
        }

        fs::write(directory.join("eula.txt"), "eula=true\n")?;
        fs::write(
            directory.join(global_args.jar_type.file_name()),
            &server_jar,
        )?;

        if has_ops {
            fs::write(directory.join("ops.txt"), &ops)?;
        }

        if has_white_list {
            fs::write(directory.join("whitelist.txt"), &white_list)?;
        }

        let properties = format!(
            "level-seed={}\nmotd={}\nquery.port={}\nserver-port={}\n{}",
            &args.level_seed, motd, port, port, &extra_props
        );

        fs::write(directory.join("server.properties"), properties)?;

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
