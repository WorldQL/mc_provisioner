use std::fs;
use std::path::PathBuf;

use color_eyre::Result;
use tracing::info;

use crate::config::{GlobalArgs, InitArgs};
use crate::utils;

pub fn init(global_args: GlobalArgs, args: InitArgs) -> Result<()> {
    let server_jar = global_args.jar_type.download(&args.jar_version)?;

    let plugins_dir = PathBuf::from("plugins");
    let bukkit_yml = PathBuf::from("bukkit.yml");
    let spigot_yml = PathBuf::from("spigot.yml");
    let paper_yml = PathBuf::from("paper.yml");

    let plugins_exists = plugins_dir.as_path().exists();
    let bukkit_exists = bukkit_yml.as_path().exists();
    let spigot_exists = spigot_yml.as_path().exists();
    let paper_exists = paper_yml.as_path().exists();

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

        if !args.skip_plugins && plugins_exists {
            copy_dir::copy_dir(&plugins_dir, directory.join(&plugins_dir))?;
        }

        if !args.no_copy_bukkit && bukkit_exists {
            fs::copy(&bukkit_yml, directory.join(&bukkit_yml))?;
        }

        if !args.no_copy_spigot && spigot_exists {
            fs::copy(&spigot_yml, directory.join(&spigot_yml))?;
        }

        if !args.no_copy_paper && paper_exists {
            fs::copy(&paper_yml, directory.join(&paper_yml))?;
        }
    }

    Ok(())
}
