<script setup lang="ts">
import { Button } from 'primevue'
import { TabRouteRootSegment, TabRouteSegment, useTabStore } from '../stores/tabs'
import { computed } from 'vue'
import { storeToRefs } from 'pinia'

type SideMenuItem = {
  readonly key: TabRouteRootSegment
  readonly icon: string
  readonly tooltip?: string
  readonly active?: boolean
  readonly disabled?: boolean
}

const ITEMS: readonly SideMenuItem[] = [
  { key: 'containers', icon: 'pi pi-box', tooltip: 'Containers' },
  { key: 'images', icon: 'pi pi-folder', tooltip: 'Images' },
  { key: 'volumes', icon: 'pi pi-database', tooltip: 'Volumes', disabled: true },
  { key: 'networks', icon: 'pi pi-globe', tooltip: 'Networks', disabled: true },
]

const store = useTabStore()
const { currentTab } = storeToRefs(store)
const { setCurrentTab } = store

const handleNavigation = (key: TabRouteSegment) => {
  setCurrentTab({ route: [key] })
}

const items = computed<readonly SideMenuItem[]>(() =>
  ITEMS.map((item) => ({
    ...item,
    active: currentTab?.value?.route[0] === item.key,
  })),
)
</script>

<template>
  <nav class="side-menu">
    <Button
      v-for="item in items"
      :key="item.key"
      v-tooltip.right="item.tooltip"
      :class="item.active ? 'side-menu-item-active' : undefined"
      :icon="item.icon"
      text
      severity="secondary"
      :aria-label="item.tooltip"
      :disabled="item.disabled"
      @click="() => handleNavigation(item.key)"
    />
  </nav>
</template>

<style scoped>
.side-menu {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  padding: 0.5rem;
  align-self: stretch;
  border-right: 1px solid var(--p-content-border-color);
  background: var(--nav-background);
}

.side-menu-item-active {
  color: var(--p-primary-color) !important;
}
</style>
