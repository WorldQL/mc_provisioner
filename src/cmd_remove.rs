use color_eyre::Result;
use tracing::{error, info};

use crate::utils;

pub fn remove(server_count: u8, start_port: u16, directory_template: String) -> Result<()> {
    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, _, directory, _) in server_iter {
        if directory.exists() && directory.is_dir() {
            info!("removing server: {:?}", &directory);

            let result = std::fs::remove_dir_all(&directory);
            if let Err(error) = result {
                error!("failed to remove directory \"{:?}\"", &directory);
                error!("{}", error);
            }
        }
    }

    Ok(())
}
