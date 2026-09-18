<script setup lang="ts">
import { AccordionContent, AccordionHeader, AccordionPanel } from 'primevue'

defineProps<{
  readonly value: string
  readonly label: string
}>()
</script>

<template>
  <AccordionPanel :value="value">
    <AccordionHeader>{{ label }}</AccordionHeader>
    <AccordionContent>
      <slot />
    </AccordionContent>
  </AccordionPanel>
</template>
