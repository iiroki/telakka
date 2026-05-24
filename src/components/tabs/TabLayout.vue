<script setup lang="ts">
import { Button, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import { computed } from 'vue'
import SideMenu from '../SideMenu.vue'
import { useTabStore } from '../../stores/tabs'
import { storeToRefs } from 'pinia'
import TabContentRouter from './TabContentRouter.vue'
import TabBreadcrumb from './TabBreadcrumb.vue'
import TabHeader from './TabHeader.vue'

const store = useTabStore()
const { tabs, selected, currentTab } = storeToRefs(store)
const active = computed<string>({
  get: () => {
    if (!selected.value) {
      throw new Error(`No selected tab found — Tabs: ${JSON.stringify(tabs.value)}`)
    }

    return selected.value
  },
  set: (v) => store.setSelected(v),
})
</script>

<!-- Figure out how to make this plus sign "sticky"? -->
<template>
  <Tabs v-model:value="active" scrollable style="height: 100%">
    <TabList>
      <Tab v-for="tab in tabs" :key="tab.id" :value="tab.id">
        <TabHeader :segment="tab.route[tab.route.length - 1]" />
        <Button
          class="tab-close"
          icon="pi pi-times"
          text
          rounded
          size="small"
          severity="secondary"
          @click.stop="store.closeTab(tab.id)"
        />
      </Tab>
      <span style="align-self: center; height: fit-content; padding-right: 0.5rem">
        <Button icon="pi pi-plus" text size="small" @click="store.createTab({ route: ['containers'] }, true)" />
      </span>
    </TabList>
    <TabPanels>
      <TabPanel v-for="tab in tabs" :key="tab.id" :value="tab.id">
        <div class="tab-body">
          <template v-if="store.isWorkspaceTab(tab)">
            <SideMenu />
            <div class="tab-body-with-menu">
              <div class="tab-breadcrumb">
                <TabBreadcrumb :route="currentTab?.route" />
              </div>
              <TabContentRouter class="tab-content" :tab="tab" />
            </div>
          </template>
          <template v-else>
            <TabContentRouter class="tab-content" :tab="tab" style="padding: 0.25rem 1rem" />
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

:deep(.p-tablist-active-bar) {
  display: none !important;
  transition: none !important;
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

.tab-breadcrumb {
  border-bottom: 1px solid var(--p-content-border-color);
  background: var(--nav-background);
}

.tab-breadcrumb :deep(.p-breadcrumb) {
  background: transparent;
  padding: 0.75rem 0.5rem;
  border: none;
}

.tab-body {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: row;
}

.tab-body-with-menu {
  flex: 1;
  min-width: 0;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.tab-content {
  flex: 1;
  min-height: 0;
  overflow: auto;
}

:deep(.p-tab) {
  position: relative;
}

:deep(.tab-close) {
  visibility: hidden;
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
}

:deep(.p-tab:hover .tab-close) {
  visibility: visible;
}
</style>
