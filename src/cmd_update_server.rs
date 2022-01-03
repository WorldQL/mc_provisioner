use std::fs;

use color_eyre::Result;

use crate::config::GlobalArgs;
use crate::utils;

pub fn update_server(global_args: GlobalArgs) -> Result<()> {
    let server_jar = global_args.jar_type.download(&global_args.jar_version)?;

    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (_, _, directory, _) in server_iter {
        if !directory.exists() {
            continue;
        }

        fs::write(
            directory.join(global_args.jar_type.file_name()),
            &server_jar,
        )?;
    }

    Ok(())
}
