<script setup lang="ts">
import { Button, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import ContainerList from './containers/ContainerList.vue'

const TAB_DEFAULT = '__default__'
</script>

<template>
  <Tabs :value="TAB_DEFAULT" scrollable class="tab-container">
    <TabList>
      <Tab :value="TAB_DEFAULT">
        <span style="font-weight: bold">Containers</span>
      </Tab>
      <Button icon="pi pi-plus" text size="small" style="align-self: center; height: fit-content" />
    </TabList>
    <TabPanels>
      <TabPanel :value="TAB_DEFAULT" style="padding-top: 0.5rem">
        <ContainerList />
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<style scoped>
.tab-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
}

:deep(.p-tabpanels) {
  padding: 0;
  flex: 1;
  min-height: 0;
  overflow: auto;
}
</style>
