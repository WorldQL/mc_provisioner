use std::fs;
use std::path::PathBuf;

use color_eyre::Result;
use tracing::warn;

pub fn init(
    server_count: u8,
    start_port: u16,
    level_seed: String,
    motd_template: String,
    skip_plugins: bool,
    no_copy_bukkit: bool,
    no_copy_spigot: bool,
    no_copy_paper: bool,
) -> Result<()> {
    if server_count == 0 {
        warn!("no servers were provisioned as --server-count was set to 0");
        return Ok(());
    }

    for idx in 1..=server_count {
        let port = start_port + (idx as u16 - 1);
        let motd = format!("{} {}", &motd_template, idx);

        let directory = motd.clone().to_lowercase().replace(' ', "_");
        let directory = PathBuf::from(directory);
        fs::create_dir(&directory)?;

        fs::write(directory.join("eula.txt"), "eula=true\n")?;

        fs::write(
            directory.join("server.properties"),
            format!(
                "level-seed={}\nmotd={}\nquery.port={}\nserver-port={}\n",
                &level_seed, motd, port, port
            ),
        )?;
    }

    todo!()
}
