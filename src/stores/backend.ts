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
import { computed, ref } from 'vue'
import { useNotificationStore } from './notification'
import { useBackendMgmtStore } from './backend-mgmt'

type UpdatedAt = {
  readonly updatedAt: Date
}

export const useBackendStore = defineStore('backend', () => {
  const status = ref<(DockerStatus & UpdatedAt) | null>(null)
  const containers = ref<DockerContainer[]>([])
  const projects = ref<DockerComposeProject[]>([])
  const stats = ref<DockerContainerStats[]>([])
  const { notify } = useNotificationStore()

  const findContainer = (id: string): DockerContainer | null => containers.value.find((c) => c.id === id) ?? null
  const findProject = (name: string): DockerComposeProject | null =>
    projects.value.find((p) => p.project === name) ?? null

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
        notify({
          level: 'warn',
          title: `Docker project command failed — Status: ${result.status}`,
          content: result.error,
          toastMs: 10_000,
        })

        return false
      }
    } catch (err) {
      notify({
        level: 'warn',
        title: 'Docker project command error',
        content: err instanceof Error ? err.message : String(err),
        toastMs: 10_000,
      })

      return false
    }

    return true
  }

  return {
    // Even though there's a circular dependency, this works since Pinia lazily resolves the stores.
    state: computed(() => useBackendMgmtStore().state),
    /** Docker status. */
    status,
    /** Docker containers — sorted by name. */
    containers,
    /** Docker Compose projects — sorted by name. */
    projects,
    /** Docker container statistics. */
    stats,
    findContainer,
    findProject,
    runContainerAction,
    runProjectAction,
  }
})
