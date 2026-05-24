<script setup lang="ts">
import { computed } from 'vue'
import { TabState } from '../../stores/tabs'
import ContainerList from '../containers/ContainerList.vue'
import ContainerPage from '../pages/ContainerPage.vue'
import SettingsPage from '../pages/SettingsPage.vue'
import TabContainerRouter from './TabContainerRouter.vue'
import ResourceMonitorPage from '../pages/ResourceMonitorPage.vue'
import NotificationPage from '../pages/NotificationPage.vue'

const { tab } = defineProps<{ readonly tab: TabState }>()
const root = computed(() => (tab.route.length > 0 ? tab.route[0] : null))
</script>

<!-- This if-else branching is not optimal but works with few routes — if-guards also work as type predicates! -->
<template>
  <div v-if="root === '_settings'">
    <SettingsPage />
  </div>
  <div v-else-if="root === '_notifications'">
    <NotificationPage />
  </div>
  <div v-else-if="root === '_resource-monitor'">
    <ResourceMonitorPage />
  </div>
  <div v-else-if="root === 'containers'">
    <template v-if="tab.route.length === 1">
      <ContainerList />
    </template>
    <template v-else-if="tab.route.length === 2">
      <TabContainerRouter :segment="tab.route[1]" />
    </template>
    <template v-else-if="tab.route[2] === 'logs'">
      <ContainerPage :id="tab.id" />
    </template>
  </div>
  <div v-else>
    <h2>!!! Unknown route: {{ tab.route.join(' -> ') }} !!!</h2>
  </div>
</template>
