import { commands, DockerComposeProject, DockerContainer } from '../tauri/bindings.gen'
import { ref } from 'vue'
import { createSafeInterval } from '../utils/timers'
import { log } from '../utils/log'
import { mergeSorted } from '../utils/sort'
import { KEY, useBackendStore } from './backend'
import { defineStore } from 'pinia'

const POLL_MS = 1000 // Make this configurable?
const BACKOFF_MS = 10000

export const useBackendMgmtStore = defineStore(`${KEY}-mgmt`, () => {
  const store = useBackendStore()
  const started = ref(false)

  let statusPollId: number | undefined
  let containerPollId: number | undefined
  let statPollId: number | undefined

  const startStatusPoller = () => {
    log.info('Starting status poller...')
    clearInterval(statusPollId)
    statusPollId = createSafeInterval(
      async () => {
        const result = await commands.getStatus()
        if (result.status === 'ok') {
          store.status = { ...result.data, updatedAt: new Date() }
        } else {
          log.warn(`Docker status error: ${result.error}`)
        }
      },
      POLL_MS,
      { instant: true },
    )
  }

  const startContainerPoller = () => {
    log.info('Starting container poller...')
    clearInterval(containerPollId)
    containerPollId = createSafeInterval(
      async () => {
        const result = await commands.getContainers()
        if (result.status === 'ok') {
          store.containers = mergeContainers(store.containers, result.data.containers)
          store.projects = mergeProjects(store.projects, result.data.projects)
        } else {
          log.warn(`Docker containers error: ${result.error}`)
        }
      },
      POLL_MS,
      { instant: true },
    )
  }

  // Dockers stats will block until the next batch is ready — we can safely keep spamming the command
  const startStatLoop = () => {
    log.info('Starting stat loop...')
    clearInterval(statPollId)
    statPollId = createSafeInterval(
      async () => {
        const result = await commands.getContainerStats()
        if (result.status === 'ok') {
          store.stats = result.data
        } else {
          log.warn(`Docker stats error: ${result.error}`)
          throw new Error(result.error)
        }
      },
      0,
      { instant: true, backoffMs: BACKOFF_MS },
    )
  }

  const start = () => {
    if (started.value) {
      return
    }

    log.info('Starting backend services...')
    started.value = true
    startStatusPoller()
    startStatLoop()
    startContainerPoller()
    log.info('Backend services started')
  }

  const stop = () => {
    log.info('Stopping backend services...')
    started.value = false
    clearInterval(statusPollId)
    clearInterval(containerPollId)
    clearInterval(statPollId)
    log.info('Backend services stopped')
  }

  return { started, start, stop }
})

/**
 * Two-pointer merge — assumes both lists sorted by `name` ascending.
 * Preserves object identity for containers whose receipt is unchanged.
 */
const mergeContainers = (current: DockerContainer[], updated: DockerContainer[]): DockerContainer[] =>
  mergeSorted(
    current,
    updated,
    (a, b) => a.name.localeCompare(b.name),
    (a, b) => a.receipt === b.receipt,
  )

/**
 * Two-pointer merge — assumes both lists sorted by `project` ascending.
 * Preserves object identity for projects whose state is unchanged.
 */
const mergeProjects = (current: DockerComposeProject[], updated: DockerComposeProject[]): DockerComposeProject[] =>
  mergeSorted(
    current,
    updated,
    (a, b) => a.project.localeCompare(b.project),
    (a, b) => a.state === b.state,
  )
