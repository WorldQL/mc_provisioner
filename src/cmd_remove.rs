use color_eyre::Result;
use tracing::{error, info};

use crate::config::GlobalArgs;
use crate::utils;

pub fn remove(global_args: GlobalArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

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
