<script setup lang="ts">
import { Button, Column, DataTable, IconField, InputIcon, InputText } from 'primevue'
import { computed, onUnmounted, ref, StyleValue, type ComponentPublicInstance } from 'vue'
import { storeToRefs } from 'pinia'
import {
  DockerComposeProject as ProjectData,
  DockerContainer,
  DockerContainerAction,
  DockerProjectAction,
} from '../../tauri/bindings.gen.ts'
import { useBackendStore } from '../../stores/backend.ts'
import ContainerStateIcon from './ContainerStateIcon.vue'
import { useNotificationStore } from '../../stores/notification.ts'
import ContainerActionButton from './ContainerActionButton.vue'
import TableColumnSelector from '../common/TableColumnSelector.vue'
import { useTabStore } from '../../stores/tabs.ts'
import { useSettingsStore } from '../../stores/settings.ts'
import { useUiStore, zTableOptions } from '../../stores/ui.ts'
import { useShortcut } from '../../composables/shortcuts.ts'
import { log } from '../../utils/log.ts'
import { createStyle } from '../../utils/style.ts'
import type { TableColumn } from '../types.ts'

/** Container data with only the structural fields. */
type ContainerData = Pick<DockerContainer, 'id' | 'name' | 'state' | 'image' | 'ports' | 'receiptDurable'> & {
  readonly displayName?: string
  readonly platform: string
}

/** Table row, one per container. Project containers carry their project as the group. */
type ContainerRow = {
  readonly key: string
  readonly group: string
  readonly data: ContainerData
}

type ContainerColumnKey = 'name' | 'id' | 'image' | 'platform' | 'ports'
type ContainerColumn = TableColumn<ContainerData, ContainerColumnKey>
type TooltipTarget = ContainerData | ProjectData

/** State has no column of its own, the icon is rendered as a part of the name. */
type TooltipKey = ContainerColumnKey | 'state'

/** Group of the containers without a project, always expanded and rendered without a group header. */
const UNGROUPED_KEY = '__ungrouped__'
const UI_OPTIONS_KEY = 'containers'

const MONOSPACE_STYLE = { fontFamily: 'monospace' }
const MONOSPACE_TOOLTIP_PT = { text: { style: createStyle(MONOSPACE_STYLE) } }

const { setCurrentTab, pushToCurrentTab } = useTabStore()
const backend = useBackendStore()
const { containers, projects } = storeToRefs(backend)
const { runContainerAction, runProjectAction, findContainer } = backend
const { notify } = useNotificationStore()
const ui = useUiStore()
const { settings } = storeToRefs(useSettingsStore())

const groupByProject = computed(() => settings.value.containers.groupByProject)
const projectMap = computed(() => new Map(projects.value.map((p) => [p.project, p])))

const isContainerData = (data: TooltipTarget): data is ContainerData => 'id' in data

/** DataTable slot props are untyped, so rows are narrowed back to the row type on the way in. */
const isContainerRow = (value: unknown): value is ContainerRow =>
  !!value && typeof value === 'object' && 'key' in value && 'data' in value
const getProject = (name: string): ProjectData => projectMap.value.get(name) ?? { project: name, state: 'unknown' }

const toContainerData = (c: DockerContainer, useProjectService = true): ContainerData => ({
  id: c.id,
  name: c.name,
  state: c.state,
  image: c.image,
  ports: c.ports,
  platform: `${c.platform.os}/${c.platform.architecture}`,
  receiptDurable: c.receiptDurable,
  displayName: useProjectService ? (c.compose?.service ?? c.name) : c.name,
})

/** Currently active hover tooltip. */
const hoverTooltip = ref<string | null>(null)
const buildHoverTooltip = (data: TooltipTarget, type: TooltipKey): string | null => {
  if (type === 'state') {
    // Container
    if (isContainerData(data)) {
      const live = findContainer(data.id).value
      if (!live) {
        return '???'
      }

      return ['[Container]', live.state, live.status + ' ' + live.statusDetail].filter(Boolean).join('\n')
    }

    // Project
    return ['[Project]', data.state].filter(Boolean).join('\n')
  } else if (type === 'name') {
    if (isContainerData(data)) {
      return findContainer(data.id).value?.name ?? null
    }

    return data.project
  } else if (type === 'id') {
    if (isContainerData(data)) {
      return data.id
    }
  } else if (type === 'image') {
    if (isContainerData(data)) {
      return data.image
    }
  } else if (type === 'ports') {
    if (isContainerData(data)) {
      return data.ports
        .filter((p) => !!p.hostPort)
        .map((p) => `${p.hostPort}:${p.containerPort} (${p.protocol})`)
        .join('\n')
    }
  }

  return null
}

const onHoverTooltip = (data: TooltipTarget, type: TooltipKey) => {
  hoverTooltip.value = buildHoverTooltip(data, type)
}

const handleAction = async (action: DockerContainerAction | DockerProjectAction, data: TooltipTarget) => {
  const isContainer = isContainerData(data)
  notify({
    level: 'info',
    title: `Action: ${action}, Target: ${isContainer ? (data.displayName ?? data.name) : data.project} (${isContainer ? 'container' : 'project'})`,
    toastMs: 3000,
  })

  if (isContainer) {
    await runContainerAction(action as DockerContainerAction, data.id)
  } else {
    await runProjectAction(action as DockerProjectAction, data.project)
  }
}

const selection = ref<ContainerRow | null>(null)

const handleSelect = ({ data: row }: { data: ContainerRow }) => {
  log.debug(`Container selected — ID: ${row.data.id}`)
  pushToCurrentTab(`container=${row.data.id}`)
}

const handleProjectSelect = (project: string) => {
  log.debug(`Project selected: ${project}`)
  setCurrentTab({ route: ['containers', `project=${project}`] })
}

//
// COLUMN RESIZE:
// Custom resize logic to prevent text selection while resizing columns.
// PrimeVue does not seem to provide any built-in solutions for that,
// so this is the end result of that...
//

const table = ref<ComponentPublicInstance | null>(null)
const isResizing = ref(false)

const onResizeStart = (e: PointerEvent) => {
  if ((e.target as HTMLElement).classList.contains('p-datatable-column-resizer')) {
    isResizing.value = true
  }
}

const onResizeEnd = () => {
  isResizing.value = false

  const root = table.value?.$el as HTMLElement | undefined
  const headers = [...(root?.querySelectorAll<HTMLElement>('thead > tr > th') ?? [])]
  const total = headers.reduce((sum, h) => sum + h.offsetWidth, 0)
  if (total === 0) {
    return
  }

  const widths = headers.map((h) => h.offsetWidth / total)
  ui.set(UI_OPTIONS_KEY, { columns: { widths } })
}

const preventSelect = (e: Event) => {
  if (isResizing.value) {
    e.preventDefault()
  }
}

document.addEventListener('selectstart', preventSelect)
onUnmounted(() => document.removeEventListener('selectstart', preventSelect))
ui.get(UI_OPTIONS_KEY, zTableOptions)

//
// DATA
//

/**
 * Cache of rows by container ID. Rows can be reused when `receiptDurable` is unchanged,
 * which leads to stable row refs and less DataTable re-rendering when only transient fields change.
 */
const cache = new Map<string, ContainerRow>()

/**
 * Rows in group order: containers without a project first, then the project containers grouped together.
 *
 * DataTable renders a group header whenever the group changes, so the rows of a group must be consecutive.
 */
const rows = computed<ContainerRow[]>(() => {
  const stale = new Set(cache.keys())
  const filter = search.value.trim().toLowerCase()

  const ungrouped: ContainerRow[] = []
  const grouped = new Map<string, ContainerRow[]>()

  for (const container of containers.value) {
    const filterFields = [
      container.name,
      container.image,
      ...container.ports.map((p) => `${p.hostPort ?? ''}:${p.containerPort}`),
    ]

    if (filter && !filterFields.some((v) => v.toLowerCase().includes(filter))) {
      continue
    }

    const group = groupByProject.value ? (container.compose?.project ?? UNGROUPED_KEY) : UNGROUPED_KEY
    const cached = cache.get(container.id)
    const useCached = !!cached && cached.group === group && cached.data.receiptDurable === container.receiptDurable

    let row: ContainerRow
    if (useCached) {
      row = cached
    } else {
      row = { key: container.id, group, data: toContainerData(container, groupByProject.value) }
      cache.set(container.id, row)
    }

    stale.delete(container.id)

    if (group === UNGROUPED_KEY) {
      ungrouped.push(row)
    } else {
      const projectRows = grouped.get(group) ?? []
      projectRows.push(row)
      grouped.set(group, projectRows)
    }
  }

  for (const id of stale) {
    cache.delete(id)
  }

  return [...ungrouped, ...[...grouped.values()].flat()]
})

const expandedRowGroups = ref<string[]>([UNGROUPED_KEY])
const hasUngrouped = computed(() => rows.value.some((r) => r.group === UNGROUPED_KEY))
const rowClass = (row: ContainerRow) => (row.group === UNGROUPED_KEY ? undefined : 'datatable-child-tint')

//
// SEARCH
//

const search = ref('')
const searchInput = ref<ComponentPublicInstance | null>(null)
useShortcut({ key: 'f', mod: true }, () => (searchInput.value?.$el as HTMLInputElement | undefined)?.focus())

//
// COLUMNS
//

const COLUMNS: ContainerColumn[] = [
  { key: 'name', header: 'Name', monospace: true, locked: true, value: (d) => d.displayName ?? d.name },
  { key: 'id', header: 'ID', monospace: true, value: (d) => d.id },
  { key: 'image', header: 'Image', monospace: true, value: (d) => d.image },
  { key: 'platform', header: 'Platform', monospace: true, value: (d) => d.platform },
  {
    key: 'ports',
    header: 'Ports',
    value: (d) =>
      d.ports
        .filter((p) => !!p.hostPort)
        .map((p) => `${p.hostPort}:${p.containerPort}`)
        .join('\n'),
  },
]

const DEFAULT_VISIBLE_COLUMNS: ContainerColumnKey[] = ['name', 'image', 'ports']

const columnMap = new Map(COLUMNS.map((col) => [col.key, col]))
const columnOrder = ref<ContainerColumnKey[]>(COLUMNS.map((col) => col.key))
const visibleColumnKeys = ref<ContainerColumnKey[]>([...DEFAULT_VISIBLE_COLUMNS])

const columns = computed<ContainerColumn[]>(() =>
  columnOrder.value
    .filter((key) => visibleColumnKeys.value.includes(key))
    .flatMap((key) => columnMap.get(key) ?? [])
    .map((col, index) => ({
      ...col,
      index,
      bodyStyle: {
        ...(col.value ? { maxWidth: 0 } : undefined),
        ...(col.monospace ? MONOSPACE_STYLE : undefined),
        ...col.bodyStyle,
      },
    })),
)

/** Row actions are rendered into the last column, anchored to the right edge of the row. */
const lastColumnKey = computed(() => columns.value[columns.value.length - 1]?.key)

/** Column changes need a remount, since the resizable table keeps its column widths in inline styles. */
const columnsKey = computed(() => columns.value.map((col) => col.key).join('-'))

const OPTIONS = [{ key: 'actions', label: 'Show actions' }]

const options = ref<Record<string, boolean>>({ actions: true })
const showActions = computed(() => options.value.actions)

// Notes:
// - "maxWidth" = 0 kills content-driven column growth
</script>

<template>
  <DataTable
    ref="table"
    :key="columnsKey"
    v-model:selection="selection"
    v-model:expanded-row-groups="expandedRowGroups"
    :value="rows"
    data-key="key"
    selection-mode="single"
    expandable-row-groups
    group-rows-by="group"
    :row-group-mode="groupByProject ? 'subheader' : undefined"
    :row-class="rowClass"
    resizable-columns
    column-resize-mode="fit"
    class="container-table"
    :class="{ 'datatable-resizing': isResizing, 'ungrouped-header': hasUngrouped }"
    :style="{ '--row-actions-width': showActions ? undefined : '0rem' }"
    @row-select="handleSelect"
    @pointerdown.capture="onResizeStart"
    @column-resize-end="onResizeEnd"
  >
    <template #header>
      <div class="header">
        <IconField class="search">
          <InputIcon class="pi pi-search" />
          <InputText ref="searchInput" v-model="search" placeholder="Search" size="small" fluid />
        </IconField>

        <TableColumnSelector
          v-model:order="columnOrder"
          v-model:visible="visibleColumnKeys"
          v-model:option-values="options"
          :columns="COLUMNS"
          :default-visible="DEFAULT_VISIBLE_COLUMNS"
          :options="OPTIONS"
        />

        <Button icon="pi pi-ellipsis-v" variant="text" severity="secondary" size="small" />
      </div>
    </template>

    <Column v-for="col in columns" :key="col.key" :header="col.header" :style="col.style" :body-style="col.bodyStyle">
      <template #body="{ data: row }">
        <template v-if="isContainerRow(row)">
          <span class="cell">
            <span
              v-if="col.key === 'name'"
              v-tooltip.top="hoverTooltip"
              class="state-icon"
              @pointerenter="onHoverTooltip(row.data, 'state')"
            >
              <ContainerStateIcon
                type="container"
                :state="row.data.state"
                :style="{ fontSize: '1.5rem' } as StyleValue"
              />
            </span>

            <span
              v-if="col.value?.(row.data) != null"
              v-tooltip.top="{ value: hoverTooltip, pt: col.monospace ? MONOSPACE_TOOLTIP_PT : undefined }"
              class="cell-text"
              @pointerenter="onHoverTooltip(row.data, col.key)"
            >
              {{ col.value?.(row.data) }}
            </span>
          </span>

          <span v-if="showActions && col.key === lastColumnKey" class="row-actions">
            <ContainerActionButton :data="row.data" @action="(a) => handleAction(a, row.data)" />
          </span>
        </template>
      </template>
    </Column>

    <template #groupheader="{ data: row }">
      <span v-if="isContainerRow(row)" class="group-header">
        <span
          v-tooltip.top="hoverTooltip"
          class="state-icon"
          @pointerenter="onHoverTooltip(getProject(row.group), 'state')"
        >
          <ContainerStateIcon type="project" :state="getProject(row.group).state" :style="{ fontSize: '1.5rem' }" />
        </span>

        <span
          v-tooltip.top="{ value: hoverTooltip, pt: MONOSPACE_TOOLTIP_PT }"
          class="cell-text group-name"
          @pointerenter="onHoverTooltip(getProject(row.group), 'name')"
          @click="handleProjectSelect(row.group)"
        >
          {{ row.group }}
        </span>

        <span v-if="showActions" class="row-actions">
          <ContainerActionButton
            :data="getProject(row.group)"
            @action="(a) => handleAction(a, getProject(row.group))"
          />
        </span>
      </span>
    </template>
  </DataTable>
</template>

<style scoped>
.container-table {
  /* Space taken by the row actions: two small icon buttons */
  --row-actions-width: 4.25rem;
  --row-actions-offset: 0.5rem;
}

.header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* PrimeIcons sit slightly low in their em box next to a button label */
.header :deep(.p-button-icon) {
  transform: translateY(1px);
}

.search {
  flex: 1;
  max-width: 20rem;
  margin-right: auto;
}

/* Containers without a project are always expanded, so their group header is hidden */
.ungrouped-header :deep(.p-datatable-tbody > tr.p-datatable-row-group-header:first-child) {
  display: none;
}

:deep(.p-datatable-row-group-header > td) {
  padding-left: 0.5rem;
}

:deep(.p-datatable-row-group-header .p-datatable-row-toggle-button) {
  margin-right: 0.25rem;
}

.group-header {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  width: calc(100% - 2.75rem);
  vertical-align: middle;
}

/* State icon and name form a single cell, same as in the group header */
.cell {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.state-icon {
  display: flex;
  margin-right: 0.25rem;
}

.group-name {
  font-family: monospace;
  font-weight: 500;
}

:deep(.p-datatable-tbody > tr) {
  cursor: pointer;
}

:deep(.p-datatable-tbody > tr > td) {
  min-width: 0;
}

/* Resizable tables clip their cells, which would cut off the row actions */
:deep(.p-datatable-resizable-table > .p-datatable-tbody > tr > td) {
  overflow: visible;
}

/*
 * Actions are anchored to the last cell of the row instead of having a column of their own,
 * which keeps them at the right edge no matter how the columns are sized.
 * Table rows are not a reliable containing block for absolute positioning, so the cell is the anchor.
 */
:deep(.p-datatable-tbody > tr > td:last-child),
:deep(.p-datatable-row-group-header > td) {
  position: relative;
  /* Reserve room for the actions so the cell content is not rendered under them */
  padding-right: calc(var(--row-actions-width) + var(--row-actions-offset) * 2);
}

.row-actions {
  position: absolute;
  top: 0;
  right: var(--row-actions-offset);
  bottom: 0;
  display: flex;
  align-items: center;
}

.cell-text {
  display: block;
  min-width: 0;
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

:deep(.p-datatable-column-resizer) {
  border-right: 1px solid var(--p-content-border-color);
  opacity: 0.3;
  transition: opacity 0.15s;
  top: 20%;
  bottom: 20%;
  height: auto;
}

:deep(.p-datatable-column-resizer:hover),
:deep(.p-datatable-column-resizer-helper) {
  opacity: 1;
}

.datatable-resizing {
  user-select: none;
}
</style>
