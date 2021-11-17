use clap::Result;
use tracing::info;

use crate::config::GlobalArgs;
use crate::utils;

pub fn reset_world(global_args: GlobalArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (_, _, directory, _) in server_iter {
        if !directory.exists() {
            continue;
        }

        info!("resetting world for server: {:?}", &directory);

        let world = directory.join("world");
        if world.exists() && world.is_dir() {
            let _ = std::fs::remove_dir_all(world);
        }

        let nether = directory.join("world_nether");
        if nether.exists() && nether.is_dir() {
            let _ = std::fs::remove_dir_all(nether);
        }

        let the_end = directory.join("world_the_end");
        if the_end.exists() && the_end.is_dir() {
            let _ = std::fs::remove_dir_all(the_end);
        }
    }

    Ok(())
}
