<script setup lang="ts">
import { TabState } from '../../stores/tab'
import Todo from '../common/Todo.vue'
import ContainerList from '../containers/ContainerList.vue'
import SettingsPage from '../pages/SettingsPage.vue'

const { tab } = defineProps<{ readonly tab: TabState }>()
</script>

<!-- This if-else branching is not optimal but works with few routes — if-guards also work as type predicates! -->
<template>
  <div v-if="tab.route.key === '_settings'">
    <SettingsPage />
  </div>
  <div v-else-if="tab.route.key === '_notifications'">
    <h3>Notifications</h3>
    <Todo />
  </div>
  <div v-else-if="tab.route.key === '_resource-monitor'">
    <h3>Resource Monitor</h3>
    <Todo />
  </div>
  <div v-else-if="tab.route.key === 'containers'">
    <ContainerList />
  </div>
  <div v-else-if="tab.route.key === 'containers/:id'">
    <h2>Container — ID: {{ tab.route.id }}</h2>
  </div>
  <div v-else>
    <h2>Unknown route: {{ tab.route.key }}</h2>
  </div>
</template>
