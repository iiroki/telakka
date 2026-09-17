<script setup lang="ts">
import { AccordionContent, AccordionHeader, AccordionPanel } from 'primevue'

defineProps<{
  readonly key: string
  readonly label: string
}>()
</script>

<template>
  <AccordionPanel :value="key">
    <AccordionHeader>{{ label }}</AccordionHeader>
    <AccordionContent>
      <slot />
    </AccordionContent>
  </AccordionPanel>
</template>
