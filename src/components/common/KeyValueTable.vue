<script setup lang="ts" generic="T">
type Row<T> = {
  label: string
  value: T
}

defineProps<{
  rows: readonly Row<T>[]
  keyClass?: string
  keyStyle?: Record<string, string>
  valueClass?: string
  valueStyle?: Record<string, string>
}>()
</script>

<template>
  <table class="kv">
    <tbody>
      <tr v-for="row in rows" :key="row.label">
        <th :class="keyClass" :style="keyStyle">{{ row.label }}</th>
        <td :class="valueClass" :style="valueStyle">
          <slot name="value" :row="row">{{ row.value ?? '—' }}</slot>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.kv {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.85rem;
}

.kv th {
  text-align: left;
  font-weight: 500;
  opacity: 0.7;
  padding: 0.25rem 0.75rem 0.25rem 0;
  white-space: nowrap;
  width: 0;
}

.kv td {
  padding: 0.25rem 0;
}
</style>
