use std::process::Command;
use std::time::Duration;

use cmd_lib::run_cmd;
use color_eyre::Result;
use tracing::{error, info, warn};
use wait_timeout::ChildExt;

use crate::arg_types::ServerMemory;
use crate::config::{GlobalArgs, StartArgs};
use crate::utils;

fn generate_jvm_args(args: StartArgs) -> String {
    let mut flags = vec![];
    flags.push(format!("-Xmx{}", args.max_memory));

    if args.use_aikar_flags {
        let large_mem = args.max_memory >= ServerMemory::from("12G");

        let new_size_percent = if large_mem { 40 } else { 30 };
        let max_new_size_percent = if large_mem { 50 } else { 40 };
        let heap_region_size = if large_mem { "16M" } else { "8M" };
        let reserve_percent = if large_mem { 15 } else { 20 };
        let init_heap_occupancy_percent = if large_mem { 20 } else { 15 };

        flags.push("-XX:+UseG1GC".into());
        flags.push("-XX:+ParallelRefProcEnabled".into());
        flags.push("-XX:MaxGCPauseMillis=200".into());
        flags.push("-XX:+UnlockExperimentalVMOptions".into());
        flags.push("-XX:+DisableExplicitGC".into());
        flags.push("-XX:+AlwaysPreTouch".into());
        flags.push(format!("-XX:G1NewSizePercent={}", new_size_percent));
        flags.push(format!("-XX:G1MaxNewSizePercent={}", max_new_size_percent));
        flags.push(format!("-XX:G1HeapRegionSize={}", heap_region_size));
        flags.push(format!("-XX:G1ReservePercent={}", reserve_percent));
        flags.push("-XX:G1HeapWastePercent=5".into());
        flags.push("-XX:G1MixedGCCountTarget=4".into());
        flags.push(format!(
            "-XX:InitiatingHeapOccupancyPercent={}",
            init_heap_occupancy_percent
        ));
        flags.push("-XX:G1MixedGCLiveThresholdPercent=90".into());
        flags.push("-XX:G1RSetUpdatingPauseTimePercent=5".into());
        flags.push("-XX:SurvivorRatio=32".into());
        flags.push("-XX:+PerfDisableSharedMem".into());
        flags.push("-XX:MaxTenuringThreshold=1".into());
        flags.push("-Dusing.aikars.flags=https://mcflags.emc.gs".into());
        flags.push("-Daikars.new.flags=true".into());
    }

    if let Some(jvm_args) = args.jvm_args {
        flags.push(jvm_args)
    }

    flags.join(" ")
}

pub fn start(global_args: GlobalArgs, args: StartArgs) -> Result<()> {
    let server_iter = utils::server_iter(
        global_args.server_count,
        global_args.start_port,
        &global_args.directory_template,
    );

    let jvm_args = generate_jvm_args(args);
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

        let run = format!(
            "java {} -jar {} nogui ; tmux wait -S {}_exit",
            &jvm_args,
            global_args.jar_type.file_name(),
            &name
        );

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

        // Wait for server to shut down
        let exit_handle = format!("{}_exit", &name);
        let mut child = match Command::new("tmux").arg("wait").arg(exit_handle).spawn() {
            Ok(child) => child,
            Err(_) => {
                error!("failed to stop \"{}\"", &name);
                continue;
            }
        };

        // After N seconds, timeout and kill anyway
        let wait_duration = Duration::from_secs(u64::from(global_args.timeout_secs));
        match child.wait_timeout(wait_duration) {
            Err(_) => {
                error!("failed to stop \"{}\"", &name);
                continue;
            }

            Ok(None) => warn!("reached wait timeout, forcefully killing: {}", &name),
            _ => (),
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

    let jvm_args = generate_jvm_args(args);
    for (_, _, directory, _) in server_iter {
        let name = directory.to_str().unwrap();
        info!("restarting tmux session: {}", &name);

        if run_cmd!(tmux send -t $name C-c).is_err() {
            error!("failed to restart \"{}\"", &name);
            continue;
        }

        // Wait for server to shut down
        let exit_handle = format!("{}_exit", &name);
        let mut child = match Command::new("tmux").arg("wait").arg(exit_handle).spawn() {
            Ok(child) => child,
            Err(_) => {
                error!("failed to stop \"{}\"", &name);
                continue;
            }
        };

        // After N seconds, timeout and restart anyway
        let wait_duration = Duration::from_secs(u64::from(global_args.timeout_secs));
        match child.wait_timeout(wait_duration) {
            Err(_) => {
                error!("failed to stop \"{}\"", &name);
                continue;
            }

            Ok(None) => {
                warn!("reached wait timeout, forcefully restarting: {}", &name);
                warn!("please manually check that the restart was successful");
            }
            _ => (),
        }

        let run = format!(
            "java {} -jar {} nogui ; tmux wait -S {}_exit",
            &jvm_args,
            global_args.jar_type.file_name(),
            &name
        );

        if run_cmd!(tmux send -t $name $run ENTER).is_err() {
            error!("failed to restart \"{}\"", &name);
            continue;
        }
    }

    Ok(())
}
