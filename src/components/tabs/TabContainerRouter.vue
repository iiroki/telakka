<script setup lang="ts">
import { computed } from 'vue'
import { parseDetailSegment, TabRouteSegment } from '../../stores/tabs'
import ContainerPage from '../pages/ContainerPage.vue'
import Todo from '../common/Todo.vue'

const { segment } = defineProps<{ readonly segment: TabRouteSegment }>()
const result = computed(() => parseDetailSegment(segment))
</script>

<template>
  <template v-if="result.ok">
    <template v-if="result.detail === 'container'">
      <ContainerPage :id="result.value" />
    </template>
    <template v-else-if="result.detail === 'project'">
      <Todo :message="`Project: ${result.value}`" />
    </template>
  </template>
</template>
