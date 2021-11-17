# Minecraft Server Provisioner [![Build](https://github.com/WorldQL/provisioner/actions/workflows/build.yml/badge.svg)](https://github.com/WorldQL/provisioner/actions/workflows/build.yml)
> Provision Minecraft servers with ease using Paper and tmux. Useful for creating and managing Mammoth clusters.

## User Guide

Provisioner will do the following:
- Download the latest version of Paper for your desired Minecraft version.
- Copy any `paper.yml`, `bukkit.yml` found in the same directory as MC Provisioner.
- Create, start, and stop multiple Minecraft servers based on `provisioner.toml`

All you need to do is provide a plugins folder.

### Getting started
1. Download a `provisioner` binary from [GitHub Actions](https://github.com/WorldQL/provisioner/actions). Place it in an empty folder called "mcservers" (or whatever you want).
2. (optional) Copy `paper.yml`, `bukkit.yml`, and/or a `plugins/` folder from an existing Minecraft server with your desired configuration.
3. Create a `provisioner.toml` configuration file.

### Example provisioner.toml
```toml
[global]
server_count = 3

[init]
paper_version = "1.17.1"
level_seed = "mammoth"

[init.server_properties]
max-players="25"
view-distance="6"
spawn-protection="0"
difficulty="peaceful"
online-mode="true"
rate-limit="0"
```
This will configure three servers based on your template but with unique ports.

### Folder structure
You should have a folder that has contents that look like this:
```bash
paper.yml  plugins/  provisioner*  provisioner.toml
```

### Creating and managing servers
```bash
# Enter your Minecraft servers directory
cd mcservers
./provisioner init
./provisioner start
```

More commands can be viewed by running `./provisioner help`