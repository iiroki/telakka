import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { log } from '../utils/log'

export type TabRouteRootSegment = 'containers' | 'images' | 'volumes' | 'networks'
export type TabRouteSpecialSegment = '_settings' | '_notifications' | '_resource-monitor'
export type TabRouteDetail = 'container' | 'project'

export type TabRouteSegment =
  | TabRouteRootSegment
  | TabRouteSpecialSegment
  | `${TabRouteDetail}=${string}`
  | 'logs'
  | '_unknown'

export type TabRoute = TabRouteSegment[]

export type TabState = {
  readonly id: string
  readonly route: TabRoute
}

export type HasTabRoute = Pick<TabState, 'route'>
export type MaybeHasTabRoute = Partial<HasTabRoute>

type TabDetailRouteSegmentResultBase<TOk extends boolean> = {
  readonly ok: TOk
}

export type TabDetailRouteSegmentResult =
  | TabDetailRouteSegmentResultBase<false>
  | (TabDetailRouteSegmentResultBase<true> & { readonly detail: TabRouteDetail; readonly value: string })

export const useTabStore = defineStore('tab', () => {
  const tabs = ref<TabState[]>([])
  const selected = ref<string | null>(null)
  const currentTab = computed<TabState | null>(() => tabs.value.find((t) => t.id === selected.value) ?? null)

  const genId = () => window.crypto.randomUUID()
  const findTabIndex = (id: string) => tabs.value.findIndex((t) => t.id === id)
  const createDefaultRouteIfEmpty = () => {
    if (tabs.value.length === 0) {
      createTab({ route: ['containers'] }, true)
    }
  }

  const setSelected = (id: string) => (selected.value = id)

  const getTab = (id: string) => tabs.value.find((t) => t.id === id) ?? null

  const setTab = (id: string, state: Partial<Omit<TabState, 'id'>>) => {
    const i = findTabIndex(id)
    log.debug(`Set tab — Index: ${i}, State: ${JSON.stringify(state)}`)
    if (i !== -1) {
      tabs.value[i] = { ...tabs.value[i], ...state }
    }
  }

  const setCurrentTab = (state: Partial<Omit<TabState, 'id'>>) => {
    if (selected.value) {
      setTab(selected.value, state)
    }
  }

  const createTab = (state: Omit<TabState, 'id'>, select = false) => {
    const id = genId()
    tabs.value.push({ ...state, id })
    if (select) {
      setSelected(id)
    }
  }

  const openOrCreateTab = (route: TabRoute | TabRouteSegment) => {
    const r = typeof route === 'string' ? [route] : route
    const tab = tabs.value.find((t) => isEquivalentRoute(t.route, r))
    if (tab) {
      setSelected(tab.id)
    } else {
      createTab({ route: r }, true)
    }
  }

  const closeTab = (id: string) => {
    const i = findTabIndex(id)
    log.debug(`Close tab — Index: ${i}, ID: ${id}`)
    if (i < 0) {
      return
    }

    // Close first tab -> Select next tab
    // Close any other tab -> Select previous tab
    // Close only tab -> No selection
    const isFirst = i === 0
    const isClosingSelected = selected.value === id
    tabs.value.splice(i, 1)

    if (isClosingSelected) {
      if (tabs.value.length === 0) {
        createDefaultRouteIfEmpty()
      } else if (isFirst) {
        selected.value = tabs.value[0].id
      } else {
        selected.value = tabs.value[i - 1].id
      }
    }
  }

  const isWorkspaceTab = (tab: TabState | string): boolean => {
    const t = typeof tab === 'object' ? tab : getTab(tab)
    const root = t?.route && t.route.length > 0 ? t.route[0] : null
    const isSpecial = !!t && root?.startsWith('_')
    return !isSpecial
  }

  createDefaultRouteIfEmpty() // Ensure there's always at least one tab
  return {
    tabs,
    selected,
    currentTab,
    setSelected,
    setCurrentTab,
    getTab,
    setTab,
    createTab,
    openOrCreateTab,
    closeTab,
    isWorkspaceTab,
  }
})

export const isDetailSegment = (segment: TabRouteSegment | undefined, detail: TabRouteDetail): boolean => {
  const s = parseDetailSegment(segment, detail)
  return s.ok
}

export const parseDetailSegment = (
  segment: TabRouteSegment | undefined,
  detail?: TabRouteDetail,
): TabDetailRouteSegmentResult => {
  if (!segment) {
    return { ok: false }
  }

  const parts = segment.split('=', 2)
  const errorResult: TabDetailRouteSegmentResult = { ok: false }
  const maybeOkResult: TabDetailRouteSegmentResult =
    parts.length >= 2 ? { ok: true, detail: parts[0] as TabRouteDetail, value: parts[1] } : errorResult

  return !detail || parts[0] === detail ? maybeOkResult : errorResult
}

export const isEquivalentRoute = (a: TabRoute, b: TabRoute): boolean =>
  a.length > 0 && a.length === b.length && a.every((seg, i) => seg === b[i])
