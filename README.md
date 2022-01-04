# Minecraft Server Provisioner [![Build](https://github.com/WorldQL/provisioner/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/provisioner/actions/workflows/build.yml)
> Provision Mammoth-ready Minecraft clusters with ease!

## Overview
Provisioner is a tool designed to make the creation of Mammoth clusters a breeze.  
It is designed to automate the following:
* Downloading and installing the latest version of Paper for your desired Minecraft version.
* Setting up `server.properties` for each server.
* Copying Plugins and other arbitrary config files to each server.
* Starting and stopping each provisioned server, running each in the background.
* **And more...**

## User Guide
### Commands and Flags
| Command | Run | Usage |
| - | - | - |
| Init | `./provisioner init` | Initialise and configure each server. |
| Sync | `./provisioner sync` | Sync specified directories to all servers. |
| Reset World | `./provisioner reset-world` | Resets each server's world files. |
| Update Server | `./provisioner update-server` | Update server .jar to the latest build for a given version. |
| Remove | `./provisioner remove` | Remove all server directories. |
| Start | `./provisioner start` | Start all servers in the background. |
| Stop | `./provisioner stop` | Stop each background server process. |
| Restart  | `./provisioner restart` | Restart all servers. |
| Completions | `./provisioner completions` | Generate shell completions. |

You can also run `./provisioner help <command>` to list each commands' available flags.

### Configuration
Passing each command the same flags over and over can be tedious and prone to mistakes, so Provisioner also supports reading from a `provisioner.toml` config file. This must be placed in the directory that you are running Provisioner in.

Most config properties have a default value, and any CLI flags will always take priority. Use `./provisioner help` for a list of default values and what each flag/property does.

#### Example Configuration
```toml
[global]
jar_type = "paper"
jar_version = "1.18.1"
server_count = 3
start_port = 25565
directory_template = "Mammoth Server"
sync_dirs = ["./plugins"]

[init]
level_seed = "mammoth"
ops = ["Steve", "Alex"]
# ... white_list ...

[init.server_properties]
# Set extra `server.properties` values here, these will be the same for every server
# !! All properties must be represented as strings !!
spawn-protection = "0"
rate-limit = "0"
difficulty = "peaceful"
online-mode = "false"
```

This will configure 3 servers with ports 25565 to 25567

### Getting Started
1. Download the latest `provisioner` binary from [GitHub Actions](https://github.com/WorldQL/mc_provisioner/actions/workflows/build.yml). **Be sure to only download binaries for tagged builds.**  
  Either save it to a directory where you will be managing your servers, or add it to your PATH for global access.
2. *__Optional__: Copy `bukkit.yml`, `spigot.yml`, `paper.yml`, and/or a `plugins` directory into the directory where you will be managing the servers.*
3. Create and fill out a `provisioner.toml` config file for ease of use.
4. Run `./provisioner init` to create your servers and then `./provisioner start` to run them all in the background.

### Use in Mammoth Development
To make your life a lot easier when developing [Mammoth](https://github.com/WorldQL/mammoth), we recommend setting up a symlink from your development directory to the provisioner plugins template directory.

```sh
# Run in the template plugins/ directory
$ ln -s /path/to/mammoth/target/WorldQLClient-1.0-SNAPSHOT.jar .

# Using WSL this will start with /mnt/
```

This will keep the template in sync with the compiled plugin JAR. You can then use a handy one-liner to stop each server, update their plugins, and then restart.

```sh
$ ./provisioner stop && ./provisioner sync-plugins && ./provisioner start
```

## Community
Join our [Discord Server](https://discord.gg/tDZkXQPzEw) for news and updates surrounding WorldQL and Mammoth!
