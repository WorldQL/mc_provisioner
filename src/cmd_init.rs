use std::fs;
use std::path::PathBuf;

use color_eyre::Result;
use tracing::info;

use crate::paper;
use crate::utils::{self, ServerProperty};

pub fn init(
    server_count: u8,
    start_port: u16,
    directory_template: String,
    paper_version: String,
    level_seed: String,
    (skip_plugins, no_copy_bukkit, no_copy_spigot, no_copy_paper): (bool, bool, bool, bool),
    server_properties: Vec<ServerProperty>,
) -> Result<()> {
    let paper_jar = paper::download_paper(&paper_version)?;

    let plugins_dir = PathBuf::from("plugins");
    let bukkit_yml = PathBuf::from("bukkit.yml");
    let spigot_yml = PathBuf::from("spigot.yml");
    let paper_yml = PathBuf::from("paper.yml");

    let plugins_exists = plugins_dir.as_path().exists();
    let bukkit_exists = bukkit_yml.as_path().exists();
    let spigot_exists = spigot_yml.as_path().exists();
    let paper_exists = paper_yml.as_path().exists();

    let extra_props = server_properties
        .into_iter()
        .map(|p| format!("{}\n", p))
        .collect::<String>();

    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, port, directory, motd) in server_iter {
        info!("creating server: {:?}", &directory);
        if !directory.exists() {
            fs::create_dir(&directory)?;
        }

        fs::write(directory.join("paper.jar"), &paper_jar)?;
        fs::write(directory.join("eula.txt"), "eula=true\n")?;

        let properties = format!(
            "level-seed={}\nmotd={}\nquery.port={}\nserver-port={}\n{}",
            &level_seed, motd, port, port, &extra_props
        );

        fs::write(directory.join("server.properties"), properties)?;

        if !skip_plugins && plugins_exists {
            copy_dir::copy_dir(&plugins_dir, directory.join(&plugins_dir))?;
        }

        if !no_copy_bukkit && bukkit_exists {
            fs::copy(&bukkit_yml, directory.join(&bukkit_yml))?;
        }

        if !no_copy_spigot && spigot_exists {
            fs::copy(&spigot_yml, directory.join(&spigot_yml))?;
        }

        if !no_copy_paper && paper_exists {
            fs::copy(&paper_yml, directory.join(&paper_yml))?;
        }
    }

    Ok(())
}
