<script setup lang="ts" generic="T">
import type { CSSProperties } from 'vue'

type RowStyled = {
  readonly keyClass?: string
  readonly keyStyle?: CSSProperties
  readonly valueClass?: string
  readonly valueStyle?: CSSProperties
}

type Row<T> = RowStyled & {
  readonly label: string
  readonly value: T
}

defineProps<
  RowStyled & {
    readonly rows: readonly Row<T>[]
  }
>()
</script>

// Use Prime DataTable?
<template>
  <table class="kv-table">
    <tbody>
      <tr v-for="row in rows" :key="row.label">
        <th :class="[keyClass, row.keyClass]" :style="{ ...keyStyle, ...row.keyStyle }">
          {{ row.label }}
        </th>
        <td :class="[valueClass, row.valueClass]" :style="{ ...valueStyle, ...row.valueStyle }">
          <slot name="value" :row="row">{{ row.value ?? '—' }}</slot>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.kv-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.85rem;
}

.kv-table th {
  text-align: left;
  font-weight: 500;
  opacity: 0.7;
  padding: 0.25rem 0.75rem 0.25rem 0;
  white-space: nowrap;
  width: 0;
}

.kv-table td {
  padding: 0.25rem 0;
}
</style>
