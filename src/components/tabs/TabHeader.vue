<script setup lang="ts">
import { useTabRouteFormatter } from '../../composables/tabs'
import { TabRouteSegment } from '../../stores/tabs'

const { segment } = defineProps<{ readonly segment: TabRouteSegment }>()
const format = useTabRouteFormatter()
const formatted = format(() => segment)
</script>

<template>
  <span
    style="
      font-weight: bold;
      width: 150px;
      height: 1.1rem;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      padding: 0;
      margin: 0;
      overflow: hidden;
    "
  >
    <i
      v-if="formatted.icon"
      :class="formatted.icon"
      style="margin-right: 0.25rem; flex-shrink: 0; margin-right: 0.5rem"
    />
    <span style="white-space: nowrap; overflow: hidden; text-overflow: ellipsis">
      {{ formatted.label }}
    </span>
  </span>
</template>
