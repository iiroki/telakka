import { computed, MaybeRefOrGetter, toValue } from 'vue'
import { RouteMenuItem } from '../components/types'
import { isEquivalentRoute, TabRoute, TabRouteSegment, useTabStore } from '../stores/tabs'
import { storeToRefs } from 'pinia'
import { useTabRouteFormatter } from './tabs'

export const useBreadcrumb = (route: MaybeRefOrGetter<TabRoute | null>) => {
  const tab = useTabStore()
  const { currentTab } = storeToRefs(tab)
  const { setCurrentTab } = tab
  const format = useTabRouteFormatter()

  return computed<readonly RouteMenuItem[]>(() => {
    const resolve = (route: TabRoute, segment: TabRouteSegment): RouteMenuItem => ({
      route,
      ...format(segment).value,
    })

    const r = toValue(route)
    return (
      r
        ?.map((s, i) => resolve(r.slice(0, 1 + i), s))
        .map((s) => ({
          ...s,
          command: () => {
            if (!currentTab.value || !isEquivalentRoute(currentTab.value.route, s.route)) {
              setCurrentTab({ route: s.route })
            }
          },
        })) ?? []
    )
  })
}
