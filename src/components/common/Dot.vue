<script setup lang="ts">
defineProps<{
  readonly color: string
  readonly size: string
}>()
</script>

<template>
  <span class="dot" :style="{ background: color, width: size, height: size }" />
</template>

<style scoped>
.dot {
  border-radius: 50%;
  display: inline-block;
}
</style>
