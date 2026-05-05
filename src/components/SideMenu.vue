<script setup lang="ts">
import { Button } from 'primevue'

type SideMenuItem = {
  readonly key: string
  readonly icon: string
  readonly tooltip?: string
  readonly active?: boolean
}

const items: readonly SideMenuItem[] = [
  { key: 'containers', icon: 'pi pi-box', tooltip: 'Containers', active: true },
  { key: 'images', icon: 'pi pi-folder', tooltip: 'Images' },
  { key: 'volumes', icon: 'pi pi-database', tooltip: 'Volumes' },
  { key: 'networks', icon: 'pi pi-globe', tooltip: 'Networks' },
  // { key: 'settings', icon: 'pi pi-cog', label: 'Settings' },
]
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
