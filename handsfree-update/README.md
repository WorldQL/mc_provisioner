# Handsfree Mammoth Updates for Continuous Delivery

This is a Node.js script which automatically updates a Mammoth deployment. It does the following:

1. Runs `./provisioner stop` in your defined MC servers directory.
1. Downloads the latest versions of mammoth and worldql_server from GitHub Actions.
2. Starts/restarts WorldQL. 
3. runs `./provisioner sync-plugins && ./provisioner start`

We run this script on a schedule using cron to have a developer Mammoth server which periodically updates to the latest pushed build.

## User guide
Run `yarn install` and create a `.env` file with the following configuration:
```env
GITHUB_ACCESS_TOKEN=ghp_youraccesstoken
SERVERS_DIRECTORY=/path/to/your/mc/servers
WORLDQL_DIRECTORY=/path/to/worldql/directory
```
