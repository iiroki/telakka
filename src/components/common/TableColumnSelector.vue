<script setup lang="ts" generic="K extends string">
import { Button, Checkbox, Popover } from 'primevue'
import { computed, ref, useId, type Ref } from 'vue'
import type { TableColumnBase, TableColumnState } from '../types.ts'

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

/** Columns in the order they are rendered in, each with its visibility. */
const columnState = defineModel<TableColumnState<K>[]>('columnState', { required: true })

/** Option values by option key. */
const optionValues = defineModel<Record<string, boolean>>('optionValues', { default: () => ({}) })

const id = useId()
const popover = ref<InstanceType<typeof Popover> | null>(null)

/** Changes are saved as drafts until applied, so closing the popover discards them. */
const draftState = ref([...columnState.value]) as Ref<TableColumnState<K>[]>
const draftOptions = ref({ ...optionValues.value })

const columnMap = computed(() => new Map(props.columns.map((c) => [c.key, c])))

/** Draft state joined with the column definitions, both visible and hidden ones. */
const items = computed(() =>
  draftState.value.flatMap((state) => {
    const column = columnMap.value.get(state.key)
    return column ? [{ ...column, visible: state.visible }] : []
  }),
)

/** Locked columns cannot be hidden. Fields the selector does not manage, such as the width, are kept as they are. */
const withLocked = (state: readonly TableColumnState<K>[]): TableColumnState<K>[] =>
  state.map((s) => ({ ...s, visible: s.visible || !!columnMap.value.get(s.key)?.locked }))

const serialize = (state: readonly TableColumnState<K>[]) => state.map((s) => `${s.key}:${s.visible}`).join()

const isDirty = computed(
  () =>
    serialize(draftState.value) !== serialize(columnState.value) ||
    (props.options ?? []).some((o) => !!draftOptions.value[o.key] !== !!optionValues.value[o.key]),
)

const hasVisible = computed(() => draftState.value.some((s) => s.visible))

const toggle = (event: Event) => popover.value?.toggle(event)

const syncDraft = () => {
  draftState.value = withLocked(columnState.value)
  draftOptions.value = { ...optionValues.value }
}

const apply = () => {
  columnState.value = withLocked(draftState.value)
  optionValues.value = { ...draftOptions.value }
  popover.value?.hide()
}

/** Locked columns stay in place, which also blocks their neighbours from moving past them. */
const canMove = (index: number, offset: number) => {
  const to = index + offset
  const columns = items.value
  return to >= 0 && to < columns.length && !columns[index].locked && !columns[to].locked
}

const move = (index: number, offset: number) => {
  if (!canMove(index, offset)) {
    return
  }

  const state = [...draftState.value]
  const [moved] = state.splice(index, 1)
  state.splice(index + offset, 0, moved)
  draftState.value = state
}

const setVisible = (index: number, visible: boolean) => {
  draftState.value = draftState.value.map((state, i) => (i === index ? { ...state, visible } : state))
}

const reset = () => {
  const visibleKeys = props.defaultVisible ?? props.columns.map((c) => c.key)
  draftState.value = withLocked(props.columns.map((c) => ({ key: c.key, visible: visibleKeys.includes(c.key) })))
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
          <Checkbox
            :model-value="col.visible"
            :input-id="`${id}-${col.key}`"
            :disabled="col.locked"
            binary
            @update:model-value="setVisible(index, !!$event)"
          />
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
        <Button icon="pi pi-check" label="Apply" size="small" :disabled="!isDirty || !hasVisible" @click="apply" />
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

.selector-items,
.selector-options {
  display: flex;
  flex-direction: column;
  padding: 0.25rem;
}

.selector-items {
  max-height: 20rem;
  overflow: auto;
}

.selector-options {
  border-top: 1px solid var(--p-content-border-color);
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

.selector-item label {
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
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

.selector-footer {
  display: flex;
  justify-content: flex-end;
  padding: 0.5rem;
  border-top: 1px solid var(--p-content-border-color);
}

/* PrimeIcons sit slightly low in their em box next to a button label */
.selector :deep(.p-button-icon) {
  transform: translateY(1px);
}
</style>
