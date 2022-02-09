# Minecraft Server Provisioner [![Build](https://github.com/WorldQL/provisioner/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/provisioner/actions/workflows/build.yml)
> Provision Mammoth-ready Minecraft clusters with ease!

## Overview
Provisioner is a tool designed to make the creation of Mammoth clusters a breeze.  
It is designed to automate the following:
* Downloading and installing the latest version of Paper (and supported forks) for your desired Minecraft version.
* Setting up `server.properties` for each server.
* Copying Plugins and other arbitrary config files to each server.
* Starting and stopping each provisioned server, running each in the background.
* **And more...**

## User Guide
### Quickstart
0. Make sure you have `tmux` installed. You can usually install this using your distro's package manager.
1. Download the latest `provisioner` binary from [GitHub Releases](https://github.com/WorldQL/mc_provisioner/releases).  
  Either save it to a directory where you will be managing your servers, or add it to your PATH for global access.
2. *__Optional__: Copy a `plugins` directory into the directory where you will be managing the servers.*
3. *__Optional__: Create a `config` directory for non-plugin configuration files you want to copy*. (bukkit.yml, paper.yml, pufferfish.yml, etc.)
4. Create and fill out a `provisioner.toml` config file for ease of use.
5. Run `./provisioner init` to create your servers and then `./provisioner start` to run them all in the background.

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
| Restart | `./provisioner restart` | Restart all servers. |
| Combine | `./provisioner combine` | Merge all world region files into a single folder. |
| Prune | `./provisioner prune` | Remove irrelevant world files from each server. |
| Completions | `./provisioner completions` | Generate shell completions. |

You can also run `./provisioner help <command>` to list each commands' available flags.

### Configuration
Passing each command the same flags over and over can be tedious and prone to mistakes, so Provisioner also supports reading from a `provisioner.toml` config file. This must be placed in the directory that you are running Provisioner in.

Most config properties have a default value, and any CLI flags will always take priority. Use `./provisioner help` for a list of default values and what each flag/property does.

#### Example Configuration
```toml
[global]
# Server .jar type and game version
jar_type = "paper"
jar_version = "1.18.1"
# Server Config
server_count = 3
start_port = 25565
level_name = "world"
directory_template = "Mammoth Server"
# Directories to sync
sync_dirs = ["./plugins"]

[init]
# Initial server config
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

This will configure 3 servers using Paper with ports 25565 to 25567.

### Syncing Files to each Server
Provisioner supports syncing files to each server, with the `--sync-dir` flag which can be repeated, or the `sync_dirs = []` config option. By default, Provisioner will sync the `./plugins` directory relative to the directory where you are using the command. Specifying your own directories will overwrite the default, so be sure to include `./plugins` in your config if you wish to keep using that directory.

The file tree inside each specified directory will be copied into each server's root. For example, if you specify `./config` as a sync directory and a file at `./config/bukkit.yml` inside, `bukkit.yml` will be copied into the server and be used as Bukkit config.

The exception to this is any directory named `plugins`. This will directly sync into each server's plugins directory, and supports a clean sync. Running `./provisioner sync --clear-plugins` will remove any top-level `.jar` files in each server's plugins directory before syncing.

### Alternate Server .jar Files
By default Provisioner will download and use [Paper](https://papermc.io/) server .jar files. If you wish to use an alternate server .jar file, you can use the `--jar-type` global flag to specify an alternate .jar type.

**Currently supported server .jar files:**
* `paper`
* [`pufferfish`](https://github.com/pufferfish-gg/Pufferfish)

Using the `--jar-version` flag or `jar_version` config option, Provisioner will query the build API of the selected jar type and download the latest available build for the specified version. **Please note that not all jar types follow the same versioning scheme for game versions.**

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
