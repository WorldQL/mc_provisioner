const { Octokit } = require("@octokit/rest");
const fs = require('fs');
const shell = require('shelljs');
require('dotenv').config()

const octokit = new Octokit({
  auth: process.env.GITHUB_ACCESS_TOKEN,
  userAgent: "Mammoth Update Bot v1",
  previews: ['jean-grey', 'symmetra'],
  timeZone: 'US/Denver',
  baseUrl: 'https://api.github.com',
  log: {
    debug: () => { },
    info: () => { },
    warn: console.warn,
    error: console.error
  },
  request: {
    agent: undefined,
    fetch: undefined,
    timeout: 0
  }
});

async function downloadLatestArtifactFromRepo(owner, repo) {
  const result = await octokit.request('GET /repos/{owner}/{repo}/actions/artifacts', {
    owner,
    repo
  });
  const latestArtifactId = result.data.artifacts[0].id;
  const downloadArtifact = await octokit.request('GET /repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}', {
    owner,
    repo,
    artifact_id: latestArtifactId,
    archive_format: 'zip'
  });
  return downloadArtifact.data;
}

(async () => {
  shell.exec("mkdir -p workdir");
  shell.exec("rm -rf ./workdir/*")
  console.log("Downloading artifacts...")
  fs.writeFileSync('./workdir/worldql.zip', Buffer.from(
    await downloadLatestArtifactFromRepo("worldql", "worldql_server")
  ));
  fs.writeFileSync('./workdir/mammoth.zip', Buffer.from(
    await downloadLatestArtifactFromRepo("worldql", "mammoth")
  ));
  fs.writeFileSync('./workdir/mc_provisioner.zip', Buffer.from(
    await downloadLatestArtifactFromRepo("worldql", "mc_provisioner")
  ));
  console.log("Unzipping artifacts...")
  shell.cd("workdir");
  shell.exec("unzip mammoth.zip && unzip mc_provisioner.zip && unzip worldql.zip");
  console.log("Setting executable bit for downloaded artifacts...");
  shell.exec("chmod +x provisioner && chmod +x worldql_server");
})();