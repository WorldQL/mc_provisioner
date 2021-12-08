import dotenv from 'dotenv'
import { execa } from 'execa'
import { lstat } from 'node:fs/promises'

// Load .env file
dotenv.config()

const dirExists = async (path) => {
  try {
    const stats = await lstat(path)
    return stats.isDirectory()
  } catch {
    return false
  }
}

const tmuxSessionExists = async (name) => {
  try {
    await execa('tmux', ['has-session', '-t', name])

    // Exit code 0, session exists
    return true
  } catch (err) {
    // Exit code 1, session does not exist
    return false
  }
}

const sleep = ms => new Promise(resolve => {
  setTimeout(() => resolve(), ms)
})

;(async () => {
  const log = (...args) => console.log('[restart]', ...args)

  const wqlDir = process.env.WORLDQL_DIRECTORY ?? './worldql'
  const serversDir = process.env.SERVERS_DIRECTORY ?? './servers'

  // Ensure `worldql` directory exists
  if (await dirExists(wqlDir) === false) {
    throw new Error(`directory "${wqlDir}" does not exist`)
  }

  // Ensure servers directory exists
  if (await dirExists(serversDir) === false) {
    throw new Error(`directory "${serversDir}" does not exist`)
  }

  // Stop running servers
  log('stopping minecraft servers')
  await execa('./provisioner', ['stop'], { cwd: serversDir })

  // Wait for a bit for servers to save data to WorldQL
  log('waiting for minecraft servers to save data')
  await sleep(5000)

  // Handle WorldQL Server
  const wqlTmuxSession = 'deployment_worldql'
  if (await tmuxSessionExists(wqlTmuxSession)) {
    log('worldql tmux session exists, sending restart')

    // Send CTRL+C to stop existing process
    await execa('tmux', ['send', '-t', wqlTmuxSession, 'C-c'])

    // Start WorldQL Server
    await execa('tmux', ['send', '-t', wqlTmuxSession, './worldql_server', 'ENTER'])
  } else {
    log('creating new worldql tmux session')

    // Create new tmux session
    await execa('tmux', ['new', '-d', '-s', wqlTmuxSession], { cwd: wqlDir })

    // Start WorldQL Server
    await execa('tmux', ['send', '-t', wqlTmuxSession, './worldql_server', 'ENTER'])
  }

  // Sync plugins directory
  log('syncing plugins directory to servers')
  await execa('./provisioner', ['sync-plugins'], { cwd: serversDir })

  // Start servers
  log('restarting minecraft servers')
  await execa('./provisioner', ['start'], { cwd: serversDir })
  //#endregion
})().catch(console.error)
