<script setup lang="ts">
import BottomBar from './components/BottomBar.vue'
</script>

<template>
  <div class="app">
    <main class="content">
      <!-- tabs + panels come later -->
    </main>
    <BottomBar />
  </div>
</template>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}

.content {
  flex: 1;
  overflow: auto;
}
</style>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  height: 100%;
  width: 100%;
  font-family: Inter, system-ui, sans-serif;
}
</style>
