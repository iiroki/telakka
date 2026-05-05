<script setup lang="ts">
import { Breadcrumb, Button, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import ContainerList from './containers/ContainerList.vue'
import { ref } from 'vue'
import { RouteMenuItem } from './types'
import SideMenu from './SideMenu.vue'

const TAB_DEFAULT = '__default__'

// TODO: Fetch the breadcrumb items from state

const nop = ref<RouteMenuItem>({
  route: '/',
})

const testBreadcrumb = ref<RouteMenuItem>({
  label: 'Containers',
  route: '/containers',
})
</script>

<template>
  <Tabs :value="TAB_DEFAULT" scrollable style="height: 100%">
    <TabList>
      <Tab :value="TAB_DEFAULT">
        <span style="font-weight: bold">Containers</span>
      </Tab>
      <Button icon="pi pi-plus" text size="small" style="align-self: center; height: fit-content" />
    </TabList>
    <TabPanels>
      <TabPanel :value="TAB_DEFAULT">
        <div class="tab-container-layout">
          <SideMenu />
          <div class="tab-container">
            <div class="tab-container-header">
              <Breadcrumb :home="nop" :model="[testBreadcrumb]" />
            </div>
            <div class="tab-container-body">
              <ContainerList />
            </div>
          </div>
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
