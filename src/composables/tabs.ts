import { computed, ComputedRef, MaybeRefOrGetter, toValue } from 'vue'
import { useBackendStore } from '../stores/backend'
import { parseDetailSegment, TabRouteDetail, TabRouteSegment } from '../stores/tabs'

export type FormattedTabRouteSegment = {
  readonly label: string
  readonly icon?: string
  readonly tooltip?: string
  readonly class?: string
}

export type TabRouteFormatter = (segment: MaybeRefOrGetter<TabRouteSegment>) => ComputedRef<FormattedTabRouteSegment>

const UNKNOWN: FormattedTabRouteSegment = { label: '???' }

export const useTabRouteFormatter = (): TabRouteFormatter => {
  const backend = useBackendStore()

  const formatDetail = (detail: TabRouteDetail, value: string): FormattedTabRouteSegment => {
    switch (detail) {
      case 'container':
        const c = backend.findContainer(value).value
        return { label: c?.name ?? value, icon: 'pi pi-box', tooltip: c?.id, class: 'mono' }
      case 'project':
        return { label: value, icon: 'pi pi-sitemap', class: 'mono' }
      default:
        return UNKNOWN
    }
  }

  return (segment) =>
    computed(() => {
      const seg = toValue(segment)
      const s = parseDetailSegment(seg)
      if (s.ok) {
        return formatDetail(s.detail, s.value)
      }

      switch (seg) {
        case 'containers':
          return { label: 'Containers' }
        case 'images':
          return { label: 'Images' }
        case '_settings':
          return { label: 'Settings' }
        case '_notifications':
          return { label: 'Notifications' }
        case '_resource-monitor':
          return { label: 'Resource Monitor' }
        default:
          return UNKNOWN
      }
    })
}
