<script setup lang="ts">
import { Column, TreeTable } from 'primevue'
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import {
  DockerComposeProject,
  DockerContainer,
  DockerContainerAction,
  DockerProjectAction,
} from '../tauri/bindings.gen'
import type { TreeNode } from 'primevue/treenode'
import { useBackendStore } from '../stores/backend'
import ContainerStateIcon from './common/ContainerStateIcon.vue'
import ContainerActionButton from './common/ContainerActionButton.vue'
import { useNotificationStore } from '../stores/notification'

type KnownKeys<T> = {
  [K in keyof T as string extends K ? never : number extends K ? never : K]: T[K]
}

type TypedTreeNode<T> = Omit<KnownKeys<TreeNode>, 'data'> & {
  readonly data: T
}

type ContainerNodeData = DockerContainer | DockerComposeProject
type ContainerNode = TypedTreeNode<ContainerNodeData>

type TooltipKey = 'state'
type ContainerListState = {
  readonly nodes: ContainerNode[]
  /** Node key -> Tooltip key -> Tooltip value */
  readonly tooltips: Map<string, Map<TooltipKey, string>>
}

const PROJECT_PREFIX = '__project__'

const backend = useBackendStore()
const { containers, projects } = storeToRefs(backend)
const { runContainerAction, runProjectAction } = backend
const { notify } = useNotificationStore()

const isContainerTreeNode = (node: ContainerNode): node is ContainerNode => 'state' in node.data
const isContainerNodeData = (data: ContainerNodeData): data is DockerContainer => 'id' in data

const isProjectNodeData = (data: ContainerNodeData): data is DockerComposeProject => 'project' in data
const isProjectNode = (node: TreeNode): node is TypedTreeNode<DockerComposeProject> => isProjectNodeData(node.data)

const getDisplayName = (data: ContainerNodeData): string => {
  if (isContainerNodeData(data)) {
    // IDEA: If project grouping is off, we should not use the service name as the display name!
    return data.compose?.service ?? data.name
  }

  return isProjectNodeData(data) ? data.project : '???'
}

const handleAction = async (action: DockerContainerAction | DockerProjectAction, data: ContainerNodeData) => {
  notify({
    level: 'info',
    title: `TODO — Action: ${action}, Target: ${getDisplayName(data)} (${isContainerNodeData(data) ? 'container' : 'project'})`,
    toastMs: 3000,
  })

  if (isContainerNodeData(data)) {
    await runContainerAction(action as DockerContainerAction, data.id)
  } else if (isProjectNodeData(data)) {
    await runProjectAction(action as DockerProjectAction, data.project)
  }
}

const state = computed<ContainerListState>(() => {
  const projectMap = new Map<string, DockerComposeProject>()
  projects.value.forEach((p) => projectMap.set(p.project, p))

  // IDEA: Make it possible to toggle the project grouping on/off?
  const nodes: ContainerNode[] = []
  for (const container of containers.value) {
    let projectNode: TypedTreeNode<DockerComposeProject> | undefined
    if (container.compose) {
      const key = PROJECT_PREFIX + container.compose.project
      const node = nodes.find((i) => i.key === key)
      if (node && isProjectNode(node)) {
        projectNode = node
      } else {
        const project = projectMap.get(container.compose.project)
        projectNode = {
          key,
          data: project ?? { project: '???', state: 'unknown' }, // Do we need to handle missing projects?
          leaf: false,
        }

        nodes.push(projectNode)
      }
    }

    const containerNode: TypedTreeNode<DockerContainer> = {
      key: container.name,
      data: container,
    }

    if (projectNode) {
      // Container belongs to a group
      if (!projectNode.children) {
        projectNode.children = []
      }

      projectNode.children.push({ ...containerNode, styleClass: 'treetable-child-tint' })
    } else {
      // Standalone container
      nodes.push(containerNode)
    }
  }

  return { nodes, tooltips: new Map() } // TODO: Tooltips
})
</script>

// TODO: Make the split button main action start/stop based on the state // TODO: Child row tint
<template>
  <TreeTable :value="state.nodes" :indentation="0">
    <Column :expander="true" :style="{ width: '1%' }" />
    <Column header="State" :style="{ width: '1%' }" :header-style="{ textAlign: 'center' }">
      <template #body="{ node }">
        <template v-if="isContainerTreeNode(node)">
          <span
            v-tooltip.top="
              [
                node.data.state,
                // isContainerNodeData(node.data) ? `${node.data.status} — ${node.data.statusDetail}` : null,
                isContainerNodeData(node.data) ? node.data.status : null,
                isContainerNodeData(node.data) ? node.data.statusDetail : null,
              ]
                .filter(Boolean)
                .join('\n')
            "
            :style="{ display: 'flex', justifyContent: 'center', width: '100%' }"
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
    <Column
      header="Name"
      :field="(data: ContainerNodeData) => getDisplayName(data)"
      :body-style="{ fontFamily: 'monospace' }"
    />
    <Column
      header="Image"
      :field="(data: ContainerNodeData) => (isContainerNodeData(data) ? data.image : '')"
      :body-style="{ fontFamily: 'monospace' }"
    />
    <Column header="Actions">
      <template #body="{ node }">
        <template v-if="isContainerTreeNode(node)">
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
</style>
