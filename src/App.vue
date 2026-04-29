<script setup lang="ts">
import { Toast } from 'primevue'
import BottomBar from './components/BottomBar.vue'
import { useBackendMgmtStore } from './stores/backend-mgmt'
import { onMounted, onUnmounted } from 'vue'
import TabContainer from './components/TabContainer.vue'

const backendMgmt = useBackendMgmtStore()
onMounted(() => {
  backendMgmt.start()
})

onUnmounted(() => {
  backendMgmt.stop()
})
</script>

<template>
  <div class="app">
    <main>
      <TabContainer />
    </main>
    <BottomBar />
  </div>
  <Toast />
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

main {
  flex: 1;
  overflow: auto;
}
</style>

<style>
html {
  font-size: 14px;
}

html,
body,
#app {
  margin: 0;
  padding: 0;
  height: 100%;
  font-family: 'Inter Variable', 'Inter', sans-serif;
  -webkit-font-smoothing: auto;
}

h1,
h2,
h3,
h4,
h5,
h6 {
  font-weight: 500;
  letter-spacing: -0.01em;
}

.p-dialog-title {
  font-weight: 500 !important;
}

.p-treetable-tbody > tr.treetable-child-tint > td {
  background-color: color-mix(in srgb, var(--p-text-color) 5%, var(--p-content-background));
}

.p-treetable-tbody > tr:hover > td {
  background-color: color-mix(in srgb, var(--p-text-color) 12%, var(--p-content-background));
}
</style>
