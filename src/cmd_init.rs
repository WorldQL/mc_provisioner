use std::fs;
use std::path::PathBuf;

use color_eyre::Result;
use tracing::warn;

pub fn init(
    server_count: u8,
    start_port: u16,
    motd_template: String,
    level_seed: String,
    skip_plugins: bool,
    (no_copy_bukkit, no_copy_spigot, no_copy_paper): (bool, bool, bool),
) -> Result<()> {
    if server_count == 0 {
        warn!("no servers were provisioned as --server-count was set to 0");
        return Ok(());
    }

    let plugins_dir = PathBuf::from("plugins");
    let bukkit_yml = PathBuf::from("bukkit.yml");
    let spigot_yml = PathBuf::from("spigot.yml");
    let paper_yml = PathBuf::from("paper.yml");

    let plugins_exists = plugins_dir.as_path().exists();
    let bukkit_exists = bukkit_yml.as_path().exists();
    let spigot_exists = spigot_yml.as_path().exists();
    let paper_exists = paper_yml.as_path().exists();

    for idx in 1..=server_count {
        let port = start_port + (idx as u16 - 1);
        let motd = format!("{} {}", &motd_template, idx);

        let directory = motd.clone().to_lowercase().replace(' ', "_");
        let directory = PathBuf::from(directory);

        if !directory.exists() {
            fs::create_dir(&directory)?;
        }

        fs::write(directory.join("eula.txt"), "eula=true\n")?;
        fs::write(
            directory.join("server.properties"),
            format!(
                "level-seed={}\nmotd={}\nquery.port={}\nserver-port={}\n",
                &level_seed, motd, port, port
            ),
        )?;

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
