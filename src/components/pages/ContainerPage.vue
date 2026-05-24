<script setup lang="ts">
import { useContainer } from '../../composables/containers'
import PageContainer from '../common/PageContainer.vue'

const { id } = defineProps<{ readonly id: string }>()
const container = useContainer(id)
</script>

<template>
  <PageContainer>
    <template v-if="container">
      <h2 style="font-family: monospace">{{ container.name }}</h2>
      <span>ID: {{ container.id }}</span>
      <span>Image: {{ container.image }}</span>
    </template>
    <template v-else>
      <span>CONTAINER NOT FOUND</span>
    </template>
  </PageContainer>
</template>
