import { defineStore } from 'pinia'
import { ref } from 'vue'

type TabRouteBase<T extends string> = {
  readonly key: T
}

type TabSpecialRoute = TabRouteBase<'_settings'> | TabRouteBase<'_notifications'> | TabRouteBase<'_resource-monitor'>
type TabWorkspaceRoute = TabRouteBase<'containers'> | TabContainerRoute
export type TabRoute = TabWorkspaceRoute | TabSpecialRoute | TabRouteBase<'_unknown'>
export type TabRouteKey = TabRoute['key']

type TabContainerRoute = TabRouteBase<'containers/:id'> & {
  readonly id: string
}

export type TabState = {
  readonly id: string
  readonly route: TabRoute
  // Add the rest here...
}

export const useTabStore = defineStore('tab', () => {
  const tabs = ref<TabState[]>([])
  const selected = ref<string | null>(null)

  const genId = () => window.crypto.randomUUID()

  const setSelected = (id: string) => (selected.value = id)

  const getTab = (id: string) => tabs.value.find((t) => t.id === id) ?? null

  const setTab = (id: string, state: Partial<Omit<TabState, 'id'>>) => {
    const i = tabs.value.findIndex((t) => t.id === id)
    if (i !== -1) {
      tabs.value[i] = { ...tabs.value[i], ...state }
    }
  }

  const createTab = (state: Omit<TabState, 'id'>, select = false) => {
    const id = genId()
    tabs.value.push({ ...state, id })
    if (select) {
      setSelected(id)
    }
  }

  const openOrCreateTab = (route: TabRoute | TabRoute['key']) => {
    const r: TabRoute = typeof route === 'object' ? route : ({ key: route } as TabRoute)
    const tab = tabs.value.find((t) => t.route.key === r.key)
    if (tab) {
      setSelected(tab.id)
    } else {
      createTab({ route: r }, true)
    }
  }

  // TODO: Remove tab

  const isWorkspaceTab = (tab: TabState | string): boolean => {
    const t = typeof tab === 'object' ? tab : getTab(tab)
    const isSpecial = !!t && t.route.key.startsWith('_')
    return !isSpecial
  }

  return { tabs, selected, setSelected, getTab, setTab, createTab, openOrCreateTab, isWorkspaceTab }
})
