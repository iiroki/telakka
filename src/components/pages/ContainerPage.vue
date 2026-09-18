<script setup lang="ts">
import { Accordion, Button } from 'primevue'
import { useContainer } from '../../composables/containers'
import PageContainer from '../common/PageContainer.vue'
import KeyValueTable from '../common/KeyValueTable.vue'
import AccordionItem from '../common/AccordionItem.vue'

const { id } = defineProps<{ readonly id: string }>()
const container = useContainer(id)
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
      <Accordion>
        <AccordionItem value="labels" label="Labels">
          <KeyValueTable class="mono" :rows="[]" />
        </AccordionItem>
      </Accordion>
      <Button>Logs</Button>
    </template>
    <template v-else>
      <span>CONTAINER NOT FOUND</span>
    </template>
  </PageContainer>
</template>
