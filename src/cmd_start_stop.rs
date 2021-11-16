use std::thread;
use std::time::Duration;

use cmd_lib::run_cmd;
use color_eyre::Result;
use tracing::info;

use crate::utils;

pub fn start(
    server_count: u8,
    start_port: u16,
    directory_template: String,
    max_memory: String,
) -> Result<()> {
    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();
        info!("starting tmux session: {}", &name);
        run_cmd!(tmux new -d  -s $name)?;

        let cd = format!("cd ./{}", &name);
        run_cmd!(tmux send -t $name $cd ENTER)?;

        let run = format!("java -Xmx{} -jar paper.jar nogui", max_memory);
        run_cmd!(tmux send -t $name $run ENTER)?;
    }

    Ok(())
}

pub fn stop(server_count: u8, start_port: u16, directory_template: String) -> Result<()> {
    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        info!("killing tmux session: {}", &name);
        run_cmd!(tmux kill-session -t $name)?;
    }

    Ok(())
}

pub fn restart(
    server_count: u8,
    start_port: u16,
    directory_template: String,
    max_memory: String,
) -> Result<()> {
    let server_iter = utils::server_iter(server_count, start_port, &directory_template);
    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();

        info!("restarting tmux session: {}", &name);
        run_cmd!(tmux send -t $name C-c)?;

        // Wait for server to shutdown
        thread::sleep(Duration::from_millis(200));

        let run = format!("java -Xmx{} -jar paper.jar nogui", max_memory);
        run_cmd!(tmux send -t $name $run ENTER)?;
    }

    Ok(())
}
