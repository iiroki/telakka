<script setup lang="ts">
import { Breadcrumb, Button, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import { computed, ref } from 'vue'
import { RouteMenuItem } from '../types'
import SideMenu from '../SideMenu.vue'
import { useTabStore } from '../../stores/tab'
import { storeToRefs } from 'pinia'
import TabBody from './TabBody.vue'

const store = useTabStore()
if (store.tabs.length === 0) {
  store.createTab({ route: { key: 'containers' } }, true)
}

const { tabs, selected } = storeToRefs(store)
const active = computed<string>({
  get: () => {
    if (!selected.value) {
      throw new Error(`No selected tab found — Tabs: ${JSON.stringify(tabs.value)}`)
    }

    return selected.value
  },
  set: (v) => store.setSelected(v),
})

// TODO: Fetch the breadcrumb items from state

const nop = ref<RouteMenuItem>({
  route: '_unknown',
})

const testBreadcrumb = ref<RouteMenuItem>({
  label: 'Containers',
  route: 'containers',
})
</script>

<!-- Figure out how to make thes plus sign "sticky"? -->
<template>
  <Tabs v-model:value="active" scrollable style="height: 100%">
    <TabList>
      <Tab v-for="tab in tabs" :key="tab.id" :value="tab.id">
        <span style="font-weight: bold"> {{ tab.route.key }}</span>
      </Tab>
      <Button
        icon="pi pi-plus"
        text
        size="small"
        style="align-self: center; height: fit-content"
        @click="store.createTab({ route: { key: 'containers' } }, true)"
      />
    </TabList>
    <TabPanels>
      <TabPanel v-for="tab in tabs" :key="tab.id" :value="tab.id">
        <div class="tab-container-layout">
          <template v-if="store.isWorkspaceTab(tab)">
            <SideMenu />
            <div class="tab-container">
              <div class="tab-container-header">
                <Breadcrumb :home="nop" :model="[testBreadcrumb]" />
              </div>
              <TabBody class="tab-container-body" :tab="tab" />
            </div>
          </template>
          <template v-else>
            <TabBody class="tab-container-body" :tab="tab" style="padding: 0.25rem 1rem" />
          </template>
        </div>
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<style scoped>
:deep(.p-tablist) {
  background: var(--nav-background);
}

:deep(.p-tabpanels) {
  padding: 0;
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

:deep(.p-tabpanel) {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.tab-container-header {
  border-bottom: 1px solid var(--p-content-border-color);
  background: var(--nav-background);
}

.tab-container-header :deep(.p-breadcrumb) {
  background: transparent;
  padding: 0.75rem 0.5rem;
  border: none;
}

.tab-container-layout {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: row;
}

.tab-container {
  flex: 1;
  min-width: 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.tab-container-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
}
</style>
