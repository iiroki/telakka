<script setup lang="ts">
import { Column, TreeTable } from 'primevue'
import { computed, ref } from 'vue'
import { storeToRefs } from 'pinia'
import {
  DockerComposeProject as ProjectData,
  DockerContainer,
  DockerContainerAction,
  DockerContainerComposeInfo,
  DockerProjectAction,
} from '../../tauri/bindings.gen'
import type { TreeNode } from 'primevue/treenode'
import { useBackendStore } from '../../stores/backend'
import ContainerStateIcon from './ContainerStateIcon.vue'
import { useNotificationStore } from '../../stores/notification'
import ContainerActionButton from './ContainerActionButton.vue'
import { useTabStore } from '../../stores/tabs'
import { log } from '../../utils/log'
import { createStyle } from '../../utils/style'

type KnownKeys<T> = {
  [K in keyof T as string extends K ? never : number extends K ? never : K]: T[K]
}

type TypedTreeNode<T> = Omit<KnownKeys<TreeNode>, 'data'> & {
  readonly data: T
}

/** Container data with only the structural fields. */
type ContainerData = Pick<DockerContainer, 'id' | 'name' | 'state' | 'image' | 'ports' | 'receiptDurable'> & {
  readonly displayName?: string
  readonly platform: string
}

type ContainerListNodeData = ContainerData | ProjectData
type ContainerListNode = TypedTreeNode<ContainerListNodeData>
type ContainerListNodeTooltipType = 'state' | 'name' | 'ports'

const PROJECT_PREFIX = '__project__'

const { setCurrentTab } = useTabStore()
const backend = useBackendStore()
const { containers, projects } = storeToRefs(backend)
const { runContainerAction, runProjectAction, findContainer, findProject } = backend
const { notify } = useNotificationStore()

const isContainerListNode = (node: TreeNode): node is ContainerListNode => 'state' in node.data
const isContainerData = (data: ContainerListNodeData): data is ContainerData => 'id' in data
const isContainerNode = (node: TreeNode): node is TypedTreeNode<ContainerData> => isContainerData(node.data)
const isProjectData = (data: ContainerListNodeData): data is ProjectData => 'project' in data
const isProjectNode = (node: TreeNode): node is TypedTreeNode<ProjectData> => isProjectData(node.data)

const getDisplayName = (data: ContainerListNodeData): string => {
  if (isContainerData(data)) {
    return data.displayName ?? data.name
  }

  return isProjectData(data) ? data.project : '???'
}

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

const toContainerKey = (container: DockerContainer): string => container.id
const toProjectKey = (project: string | ProjectData | DockerContainerComposeInfo): string =>
  PROJECT_PREFIX + (typeof project === 'string' ? project : project.project)

/** Currently active hover tooltip. */
const hoverTooltip = ref<string | null>(null)
const buildHoverTooltip = (
  { data }: Pick<ContainerListNode, 'data'>,
  type: ContainerListNodeTooltipType,
): string | null => {
  if (type === 'state') {
    if (isContainerData(data)) {
      const live = findContainer(data.id).value
      if (!live) {
        return null
      }

      return [live.state, live.status, live.statusDetail].filter(Boolean).join('\n')
    }
    if (isProjectData(data)) {
      const live = findProject(data.project).value
      return live?.state ?? null
    }
  } else if (type === 'name') {
    if (isContainerData(data)) {
      const live = findContainer(data.id).value
      if (!live) {
        return null
      }

      return live.name
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

const onHoverTooltip = (node: ContainerListNode, type: ContainerListNodeTooltipType) => {
  hoverTooltip.value = buildHoverTooltip(node, type)
}

const handleAction = async (action: DockerContainerAction | DockerProjectAction, data: ContainerListNodeData) => {
  notify({
    level: 'info',
    title: `TODO — Action: ${action}, Target: ${getDisplayName(data)} (${isContainerData(data) ? 'container' : 'project'})`,
    toastMs: 3000,
  })

  if (isContainerData(data)) {
    await runContainerAction(action as DockerContainerAction, data.id)
  } else if (isProjectData(data)) {
    await runProjectAction(action as DockerProjectAction, data.project)
  }
}

const handleSelect = (node: TreeNode) => {
  log.debug(`Node selected — Key: ${node.key}`)
  if (isContainerListNode(node)) {
    if (isContainerData(node.data)) {
      const { id } = node.data
      log.debug(`Container selected — ID: ${id}`)
      setCurrentTab({ route: ['containers', `container=${id}`] })
    } else if (isProjectData(node.data)) {
      const { project } = node.data
      log.debug(`Project selected: ${project}`)
      setCurrentTab({ route: ['containers', `project=${project}`] })
    }
  }
}

/**
 * Cache of nodes by key. Container nodes can be reused when `receiptDurable` is unchanged,
 * which leads to stable node refs and less TreeTable re-rendering when only transient fields change.
 */
const nodes = new Map<string, ContainerListNode>()

/**
 * Node tree hierarchy built with custom logic, see the reasoning in the comments below!
 *
 * Caching exists because PrimeVue TreeTable rebuilds row DOM (and tears down attached directives like `v-tooltip`)
 * whenever a node reference changes.
 * Reusing the same node ref across polls keeps tooltips and other DOM state alive across non-structural changes.
 */
const hierarchy = computed<ContainerListNode[]>(() => {
  // Phase 0:
  //   Assume all cached nodes stale, prune as visited.
  //   Build a project map for quick project lookups in the later phases.
  const stale = new Set(nodes.keys())
  const projectMap = new Map<string, ProjectData>()
  projects.value.forEach((p) => projectMap.set(toProjectKey(p), p))

  // Phase 1:
  //   Build container nodes or reuse ones from the cache, if durable receipts have not changed.
  //
  //   Map insertion order = encounter order from the backend (sorted by name), >=ES2015 spec guarantees the order.
  //
  //   Project containers are grouped as array values, standalone containers as single nodes.
  const ordered = new Map<string, ContainerListNode | ContainerListNode[]>()
  const changedProjects = new Set<string>()

  for (const container of containers.value) {
    const containerKey = toContainerKey(container)

    const cached = nodes.get(containerKey)
    const useCached =
      !!cached && isContainerData(cached.data) && cached.data.receiptDurable === container.receiptDurable

    let containerNode: ContainerListNode
    if (useCached) {
      containerNode = cached
    } else {
      containerNode = {
        key: containerKey,
        data: toContainerData(container),
        styleClass: container.compose ? 'treetable-child-tint' : undefined,
      }

      nodes.set(containerKey, containerNode)
    }

    if (container.compose) {
      const projectKey = toProjectKey(container.compose)
      let arr = ordered.get(projectKey)
      if (!arr || !Array.isArray(arr)) {
        arr = []
        ordered.set(projectKey, arr)
      }

      arr.push(containerNode)
      if (!useCached) {
        changedProjects.add(projectKey)
      }
    } else {
      ordered.set(containerKey, containerNode)
    }
  }

  // Phase 2:
  //   Build the final TreeNode hierarchy.
  //
  //  Standalone containers are directly pushed in to the result
  //  Project containers are grouped under a parent project node, which can be reused from the cache.
  const result: ContainerListNode[] = []
  for (const [key, value] of ordered) {
    stale.delete(key)

    // Standalone containers
    if (!Array.isArray(value)) {
      result.push(value)
      continue
    }

    // Project containers
    for (const { key: containerKey } of value) {
      stale.delete(containerKey) // Also mark the child container as non-stale
    }

    const projectKey = key
    const project = projectMap.get(projectKey) ?? { project: '???', state: 'unknown' as const }
    const cached = nodes.get(projectKey)

    const useCached =
      !!cached &&
      isProjectNode(cached) &&
      cached.data.state === project.state &&
      !changedProjects.has(projectKey) &&
      cached.children?.length === value.length

    let projectNode: ContainerListNode
    if (useCached) {
      projectNode = cached
    } else {
      projectNode = { key: projectKey, data: project, leaf: false, children: value }
      nodes.set(projectKey, projectNode)
    }

    result.push(projectNode)
  }

  // Phase 3: Prune stale nodes from the cache.
  for (const id of stale) {
    nodes.delete(id)
  }

  return result
})
</script>

<template>
  <TreeTable :value="hierarchy" :indentation="0" selection-mode="single" resizable-columns @node-select="handleSelect">
    <Column :expander="true" align-frozen="left" :style="{ width: '1%' }" />
    <Column header="State" :style="{ width: '1%' }" :header-style="{ textAlign: 'center' }">
      <template #body="{ node }">
        <template v-if="isContainerListNode(node)">
          <span
            v-tooltip.top="hoverTooltip"
            :style="{ display: 'flex', justifyContent: 'center', width: '100%' }"
            @pointerenter="onHoverTooltip(node, 'state')"
          >
            <ContainerStateIcon
              :type="isProjectNode(node) ? 'project' : 'container'"
              :state="node.data.state"
              :style="{ fontSize: '1.5rem' }"
            />
          </span>
        </template>
      </template>
    </Column>
    <Column header="Name" :body-style="{ fontFamily: 'monospace' }">
      <template #body="{ node }">
        <template v-if="isContainerListNode(node)">
          <span
            v-tooltip.top="{ value: hoverTooltip, pt: { text: { style: createStyle({ fontFamily: 'monospace' }) } } }"
            @pointerenter="onHoverTooltip(node, 'name')"
          >
            {{ getDisplayName(node.data) }}
          </span>
        </template>
      </template>
    </Column>
    <Column
      header="Image"
      :field="(data: ContainerListNodeData) => (isContainerData(data) ? data.image : '')"
      :body-style="{ fontFamily: 'monospace' }"
    />
    <Column
      header="Platform"
      :field="(data: ContainerListNodeData) => (isContainerData(data) ? data.platform : '')"
      :body-style="{ fontFamily: 'monospace' }"
    />
    <Column header="Ports">
      <template #body="{ node }">
        <template v-if="isContainerNode(node)">
          <span v-tooltip.top="{ value: hoverTooltip }" @pointerenter="onHoverTooltip(node, 'ports')">
            {{
              node.data.ports
                .filter((p) => !!p.hostPort)
                .map((p) => `${p.hostPort}:${p.containerPort}`)
                .join('\n')
            }}
          </span>
        </template>
      </template>
    </Column>
    <Column header="Actions">
      <template #body="{ node }">
        <template v-if="isContainerListNode(node)">
          <ContainerActionButton :data="node.data" @action="(a) => handleAction(a, node.data)" />
        </template>
      </template>
    </Column>
  </TreeTable>
</template>

<style scoped>
:deep(.p-treetable-tbody > tr) {
  cursor: pointer;
}

:deep(.p-treetable-column-resizer) {
  border-right: 1px solid var(--p-content-border-color);
  opacity: 0.3;
  transition: opacity 0.15s;
  top: 20%;
  bottom: 20%;
  height: auto;
}

:deep(.p-treetable-column-resizer:hover),
:deep(.p-treetable-column-resizer-helper) {
  opacity: 1;
}
</style>
