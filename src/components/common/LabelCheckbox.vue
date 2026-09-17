<script setup lang="ts">
import { Checkbox } from 'primevue'
import { useId } from 'vue'

defineProps<{
  readonly label: string
  /** Form field name — the PrimeVue `Form` tracks the value by this name instead of `v-model`. */
  readonly name?: string
}>()

const id = useId()
</script>

<template>
  <div class="label-checkbox">
    <Checkbox :input-id="id" :name="name" binary />
    <label :for="id">{{ label }}</label>
  </div>
</template>

<style scoped>
.label-checkbox {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.label-checkbox label {
  cursor: pointer;
}
</style>
