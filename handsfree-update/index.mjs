import { Octokit } from '@octokit/rest'
import dotenv from 'dotenv'
import JSZip from 'jszip'
import { makeExecutable } from 'make-executable'
import mkdirp from 'mkdirp'
import { Buffer } from 'node:buffer'
import { writeFile } from 'node:fs/promises'
import { join as joinPath } from 'node:path'
import process from 'node:process'
import { promisify } from 'node:util'
import { readPackageUpSync } from 'read-pkg-up'
import { default as rimrafSync } from 'rimraf'

// Promisify rimraf
const rimraf = promisify(rimrafSync)

// Load .env file
dotenv.config()

// Read package metadata
const pkg = readPackageUpSync()
const name = pkg.packageJson.name
const version = pkg.packageJson.version

// Init GitHub client
const octokit = new Octokit({
  auth: process.env.GITHUB_ACCESS_TOKEN,
  userAgent: `${name} v${version}`
})

const downloadLatestArtifact = async (owner, repo) => {
  const artifactsResp = await octokit.request('GET /repos/{owner}/{repo}/actions/artifacts', {
    owner,
    repo,
  })

  const artifacts = artifactsResp.data.artifacts
  if (artifacts.length === 0) {
    throw new Error(`no artifacts found for ${owner}/${repo}`)
  }

  const latest = artifacts[0]
  const artifactResp = await octokit.request('GET /repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}', {
    owner,
    repo,
    artifact_id: latest.id,
    archive_format: 'zip',
  })

  return Buffer.from(artifactResp.data)
}

const extractArtifact = async (zipped, directory) => {
  const zip = new JSZip()
  await zip.loadAsync(zipped)

  for (const entry of Object.values(zip.files)) {
    const path = joinPath(directory, entry.name)
    if (entry.dir) {
      await mkdirp(path)
    } else {
      const buf = await entry.async('nodebuffer')
      await writeFile(path, buf)
    }
  }
}

;(async () => {
  const wqlDir = process.env.WORLDQL_DIRECTORY ?? './worldql'
  const serversDir = process.env.SERVERS_DIRECTORY ?? './servers'

  // Create directories
  await mkdirp(wqlDir)
  await mkdirp(serversDir)

  const wqlJob = async () => {
    // Setup job logger
    const log = (...args) => console.log('    [worldql]', ...args)

    // Clean previous bin
    log('cleaning previous binary')
    const binPath = joinPath(wqlDir, 'worldql_server')
    await rimraf(binPath)

    // Extract new bin
    log('downloading latest artifact')
    const buf = await downloadLatestArtifact('worldql', 'worldql_server')
    log('extracting...')
    await extractArtifact(buf, wqlDir)

    // Make executable
    log('making binary executable')
    await makeExecutable(binPath)
  }

  const mammothJob = async () => {
    // Setup job logger
    const log = (...args) => console.log('    [mammoth]', ...args)

    // Clean previous plugins directory
    log('cleaning previous plugins directory')
    const pluginsDir = joinPath(serversDir, 'plugins')
    await rimraf(pluginsDir)
    await mkdirp(pluginsDir)

    // Extract new plugin
    log('downloading latest artifact')
    const buf = await downloadLatestArtifact('worldql', 'mammoth')
    log('extracting...')
    await extractArtifact(buf, pluginsDir)
  }

  const provisionerJob = async () => {
    // Setup job logger
    const log = (...args) => console.log('[provisioner]', ...args)

    // Clean previous bin
    log('cleaning previous binary')
    const binPath = joinPath(serversDir, 'provisioner')
    await rimraf(binPath)

    // Extract new bin
    log('downloading latest artifact')
    const buf = await downloadLatestArtifact('worldql', 'mc_provisioner')
    log('extracting...')
    await extractArtifact(buf, serversDir)

    // Make executable
    log('making binary executable')
    await makeExecutable(binPath)
  }

  await Promise.all([wqlJob(), mammothJob(), provisionerJob()])
})().catch(console.error)
