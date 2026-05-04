<script setup lang="ts">
import { DockerContainerState } from '../../tauri/bindings.gen'

type ContainerStateType = 'container' | 'project'

const ICONS: Record<ContainerStateType, string> = {
  container: 'pi pi-box',
  project: 'pi pi-sitemap',
} as const

// TODO: Validate these colors
const STATE_COLORS: Record<DockerContainerState, string> = {
  // Containers
  created: 'var(--p-yellow-500)',
  running: 'var(--p-green-500)',
  exited: 'var(--p-red-500)',
  restarting: 'var(--p-yellow-500)',
  paused: 'var(--p-blue-500)',
  dead: 'var(--p-gray-700)',
  removing: 'var(--p-gray-700)',
  // Projects
  partial: 'var(--p-orange-500)',
  // Other
  unknown: 'var(--p-gray-500)',
} as const

defineOptions({ inheritAttrs: false })
defineProps<{
  readonly type: ContainerStateType
  readonly state: DockerContainerState
}>()
</script>

<template>
  <span :class="ICONS[type]" :style="[{ color: STATE_COLORS[state] }, $attrs.style]" />
</template>
