import { defineStore } from 'pinia'
import {
  commands,
  DockerComposeProject,
  DockerContainer,
  DockerContainerAction,
  DockerContainerStats,
  DockerProjectAction,
  DockerStatus,
} from '../tauri/bindings.gen'
import { ref } from 'vue'
import { log } from '../utils/log'
import { useNotificationStore } from './notification'

type UpdatedAt = {
  readonly updatedAt: Date
}

export const KEY = 'backend'

export const useBackendStore = defineStore(KEY, () => {
  const status = ref<(DockerStatus & UpdatedAt) | null>(null)
  const containers = ref<DockerContainer[]>([])
  const projects = ref<DockerComposeProject[]>([])
  const stats = ref<DockerContainerStats[]>([])
  const { notify } = useNotificationStore()

  const runContainerAction = async (action: DockerContainerAction, id: string): Promise<boolean> => {
    try {
      const result = await commands.runContainerAction(action, id)
      if (result.status !== 'ok') {
        notify({
          level: 'warn',
          title: `Docker container command failed — Status: ${result.status}`,
          content: result.error,
          toastMs: 10_000,
        })

        return false
      }
    } catch (err) {
      notify({
        level: 'warn',
        title: 'Docker container command error',
        content: err instanceof Error ? err.message : String(err),
        toastMs: 10_000,
      })

      return false
    }

    return true
  }

  const runProjectAction = async (action: DockerProjectAction, project: string): Promise<boolean> => {
    try {
      const result = await commands.runProjectAction(action, project)
      if (result.status !== 'ok') {
        log.warn(`Docker project command failed: ${result.status} — ${result.error}`)
        return false
      }
    } catch (err) {
      log.error(`Docker project command error: ${err}`)
      return false
    }

    return true
  }

  return {
    status,
    containers,
    projects,
    stats,
    runContainerAction,
    runProjectAction,
  }
})
