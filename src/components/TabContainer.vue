<script setup lang="ts">
import { Button, Tab, TabList, TabPanel, TabPanels, Tabs } from 'primevue'
import ContainerList from './ContainerList.vue'

const TAB_DEFAULT = '__default__'
</script>

<template>
  <Tabs :value="TAB_DEFAULT" scrollable>
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
:deep(.p-tabpanels) {
  padding: 0;
}
</style>
