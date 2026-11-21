<script setup lang="ts">
import { ref, watch } from 'vue'
import { Button } from 'primevue'
import { useContainer } from '../../composables/containers'
import PageContainer from '../common/PageContainer.vue'
import KeyValueTable from '../common/KeyValueTable.vue'
import { useBackendStore } from '../../stores/backend'
import { DockerContainerDetails } from '../../tauri/bindings.gen'

const { id } = defineProps<{ readonly id: string }>()
const container = useContainer(id)
const backend = useBackendStore()
const details = ref<DockerContainerDetails | null>(null)

watch(
  () => id,
  async (containerId) => (details.value = await backend.getContainerDetails(containerId)),
  { immediate: true },
)
</script>

<template>
  <PageContainer>
    <template v-if="container">
      <h2 style="font-family: monospace">{{ container.name }}</h2>
      <KeyValueTable
        :rows="[
          { label: 'ID', value: container.id, valueStyle: { fontFamily: 'monospace' } },
          { label: 'Image', value: container.image, valueStyle: { fontFamily: 'monospace' } },
        ]"
      />
      <pre style="font-family: monospace; overflow: auto">{{ JSON.stringify(details, null, 2) }}</pre>
      <Button>Logs</Button>
    </template>
    <template v-else>
      <span>CONTAINER NOT FOUND</span>
    </template>
  </PageContainer>
</template>
