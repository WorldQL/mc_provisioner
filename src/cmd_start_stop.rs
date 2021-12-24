use cmd_lib::run_cmd;
use color_eyre::Result;
use tracing::{error, info};

use crate::config::{GlobalArgs, StartArgs};
use crate::utils;

pub fn start(global_args: GlobalArgs, args: StartArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();
        info!("starting tmux session: {}", &name);

        if run_cmd!(tmux new -d  -s $name).is_err() {
            error!("failed to start \"{}\"", &name);
            continue;
        }

        let cd = format!("cd ./{}", &name);
        if run_cmd!(tmux send -t $name $cd ENTER).is_err() {
            error!("failed to start \"{}\"", &name);
            continue;
        }

        let run = format!("java -Xmx{} -jar paper.jar nogui ; tmux wait -S {}_exit", &args.max_memory, &name);
        if run_cmd!(tmux send -t $name $run ENTER).is_err() {
            error!("failed to start \"{}\"", &name);
            continue;
        }
    }

    Ok(())
}

pub fn stop(global_args: GlobalArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        info!("gracefully stopping tmux session: {}", &name);
        if run_cmd!(tmux send -t $name C-c).is_err() {
            error!("failed to stop \"{}\"", &name);
            continue;
        }

        // TODO: Forcefully exit after N seconds
        let exit_handle = format!("{}_exit", &name);
        if run_cmd!(tmux wait $exit_handle).is_err() {
            error!("failed to stop \"{}\"", &name);
            continue;
        }

        if run_cmd!(tmux kill-session -t $name).is_err() {
            error!("failed to stop \"{}\"", &name);
            continue;
        }

        info!("stopped tmux session: {}", &name);
    }

    Ok(())
}

pub fn restart(global_args: GlobalArgs, args: StartArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();
        info!("restarting tmux session: {}", &name);

        if run_cmd!(tmux send -t $name C-c).is_err() {
            error!("failed to restart \"{}\"", &name);
            continue;
        }

        // Wait for server to shutdown
        let exit_handle = format!("{}_exit", &name);
        if run_cmd!(tmux wait $exit_handle).is_err() {
            error!("failed to restart \"{}\"", &name);
            continue;
        }

        let run = format!("java -Xmx{} -jar paper.jar nogui", &args.max_memory);
        if run_cmd!(tmux send -t $name $run ENTER).is_err() {
            error!("failed to restart \"{}\"", &name);
            continue;
        }
    }

    Ok(())
}
