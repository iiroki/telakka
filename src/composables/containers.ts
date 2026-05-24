import { computed, ComputedRef, MaybeRefOrGetter, toValue } from 'vue'
import { parseDetailSegment, TabRoute } from '../stores/tabs'
import { useBackendStore } from '../stores/backend'
import { DockerContainer } from '../tauri/bindings.gen'

/**
 * Attempts to extract the current container ID from the given route.
 */
export const useCurrentContainerId = (route: MaybeRefOrGetter<TabRoute | null>): ComputedRef<string | null> =>
  computed(() => {
    const r = toValue(route)
    if (!r) {
      return null
    }
    for (let i = r.length - 1; i >= 0; --i) {
      const s = parseDetailSegment(r[i], 'container')
      if (s.ok) {
        return s.value
      }
    }
    return null
  })

/**
 * Starts to track a container with the given ID, returning a reactive reference to it.
 */
export const useContainer = (id: MaybeRefOrGetter<string | null | undefined>): ComputedRef<DockerContainer | null> => {
  const backend = useBackendStore()
  return computed(() => {
    const containerId = toValue(id)
    return containerId ? backend.findContainer(containerId).value : null
  })
}
