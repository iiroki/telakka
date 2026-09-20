<script setup lang="ts" generic="K extends string">
import { Button, Checkbox, Popover } from 'primevue'
import { computed, ref, useId, type Ref } from 'vue'
import type { TableColumnBase } from '../types.ts'

/** Extra option toggle, rendered as a checkbox below the columns. */
type ExtraOption = {
  readonly key: string
  readonly label: string
  /** Value after a reset, defaults to `true`. */
  readonly default?: boolean
}

const props = defineProps<{
  readonly columns: readonly TableColumnBase<K>[]
  /** Columns visible after a reset. Defaults to all columns. */
  readonly defaultVisible?: readonly K[]
  readonly options?: readonly ExtraOption[]
}>()

/** All column keys, in the order they are rendered in. */
const order = defineModel<K[]>('order', { required: true })

/** Keys of the visible columns, order does not matter. */
const visible = defineModel<K[]>('visible', { required: true })

/** Option values by option key. */
const optionValues = defineModel<Record<string, boolean>>('optionValues', { default: () => ({}) })

const id = useId()
const popover = ref<InstanceType<typeof Popover> | null>(null)

/** Changes are saved as drafts until applied, so closing the popover discards them. */
const draftOrder = ref([...order.value]) as Ref<K[]>
const draftVisible = ref([...visible.value]) as Ref<K[]>
const draftOptions = ref({ ...optionValues.value })

const columnMap = computed(() => new Map(props.columns.map((c) => [c.key, c])))
const items = computed(() => draftOrder.value.flatMap((key) => columnMap.value.get(key) ?? []))

/** Locked columns cannot be hidden, so they are always part of the visible keys. */
const withLocked = (keys: readonly K[]): K[] => [
  ...new Set([...keys, ...props.columns.filter((c) => c.locked).map((c) => c.key)]),
]

const isDirty = computed(
  () =>
    draftOrder.value.join() !== order.value.join() ||
    draftVisible.value.filter((k) => !visible.value.includes(k)).length > 0 ||
    visible.value.filter((k) => !draftVisible.value.includes(k)).length > 0 ||
    (props.options ?? []).some((o) => !!draftOptions.value[o.key] !== !!optionValues.value[o.key]),
)

const toggle = (event: Event) => popover.value?.toggle(event)

const syncDraft = () => {
  draftOrder.value = [...order.value]
  draftVisible.value = withLocked(visible.value)
  draftOptions.value = { ...optionValues.value }
}

const apply = () => {
  order.value = [...draftOrder.value]
  visible.value = withLocked(draftVisible.value)
  optionValues.value = { ...draftOptions.value }
  popover.value?.hide()
}

const canMove = (index: number, offset: number) => {
  const to = index + offset
  return to >= 0 && to < items.value.length && !items.value[index].locked && !items.value[to].locked
}

const move = (index: number, offset: number) => {
  const to = index + offset
  if (!canMove(index, offset)) {
    return
  }

  const keys = [...draftOrder.value]
  const [moved] = keys.splice(index, 1)
  keys.splice(to, 0, moved)
  draftOrder.value = keys
}

const reset = () => {
  draftOrder.value = props.columns.map((c) => c.key)
  draftVisible.value = withLocked(props.defaultVisible ?? props.columns.map((c) => c.key))
  draftOptions.value = Object.fromEntries((props.options ?? []).map((o) => [o.key, o.default ?? true]))
}
</script>

<template>
  <Button icon="pi pi-columns-2" label="Columns" variant="text" severity="secondary" size="small" @click="toggle" />

  <Popover ref="popover" :pt="{ content: { style: { padding: 0 } } }" @show="syncDraft">
    <div class="selector">
      <div class="selector-header">
        <span class="selector-title">Columns</span>
        <Button icon="pi pi-refresh" label="Reset" variant="text" severity="secondary" size="small" @click="reset" />
      </div>

      <div class="selector-items">
        <div v-for="(col, index) in items" :key="col.key" class="selector-item">
          <Checkbox v-model="draftVisible" :value="col.key" :input-id="`${id}-${col.key}`" :disabled="col.locked" />
          <label :for="`${id}-${col.key}`">{{ col.header ?? col.key }}</label>

          <span class="selector-move">
            <Button
              icon="pi pi-chevron-up"
              variant="text"
              severity="secondary"
              size="small"
              :disabled="!canMove(index, -1)"
              @click="move(index, -1)"
            />

            <Button
              icon="pi pi-chevron-down"
              variant="text"
              severity="secondary"
              size="small"
              :disabled="!canMove(index, 1)"
              @click="move(index, 1)"
            />
          </span>
        </div>
      </div>

      <div v-if="options?.length" class="selector-options">
        <div v-for="option in options" :key="option.key" class="selector-item">
          <Checkbox v-model="draftOptions[option.key]" :input-id="`${id}-${option.key}`" binary />
          <label :for="`${id}-${option.key}`">{{ option.label }}</label>
        </div>
      </div>

      <div class="selector-footer">
        <Button
          icon="pi pi-check"
          label="Apply"
          size="small"
          :disabled="!isDirty || draftVisible.length === 0"
          @click="apply"
        />
      </div>
    </div>
  </Popover>
</template>

<style scoped>
.selector {
  width: 16rem;
}

.selector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
  padding: 0.5rem 0.5rem 0.5rem 0.75rem;
  border-bottom: 1px solid var(--p-content-border-color);
}

.selector-title {
  font-weight: 500;
}

.selector-items {
  display: flex;
  flex-direction: column;
  padding: 0.25rem;
  max-height: 20rem;
  overflow: auto;
}

.selector-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  /* Matches the height of the column rows, which is set by their move buttons */
  min-height: 2rem;
  padding: 0.125rem 0.25rem 0.125rem 0.5rem;
  border-radius: var(--p-content-border-radius);
  user-select: none;
}

.selector-item:hover {
  background-color: color-mix(in srgb, var(--p-text-color) 8%, var(--p-content-background));
}

.selector-move {
  display: flex;
  margin-left: auto;
}

.selector-move :deep(.p-button) {
  width: 1.75rem;
  height: 1.75rem;
  padding: 0;
}

.selector-options {
  display: flex;
  flex-direction: column;
  padding: 0.25rem;
  border-top: 1px solid var(--p-content-border-color);
}

.selector-footer {
  display: flex;
  justify-content: flex-end;
  padding: 0.5rem;
  border-top: 1px solid var(--p-content-border-color);
}

.selector-item label {
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* PrimeIcons sit slightly low in their em box next to a button label */
.selector :deep(.p-button-icon) {
  transform: translateY(1px);
}
</style>
