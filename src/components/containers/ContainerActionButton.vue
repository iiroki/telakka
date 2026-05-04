<script setup lang="ts">
import { ButtonGroup, Button, Menu } from 'primevue'
import { computed, ref } from 'vue'
import { DockerComposeProject, DockerContainer, DockerContainerAction } from '../../tauri/bindings.gen'
import { MenuItem } from 'primevue/menuitem'

type ContainerStateData = Pick<DockerContainer, 'state'> | Pick<DockerComposeProject, 'state' | 'project'>

const props = defineProps<{
  readonly data: ContainerStateData
}>()

const emit = defineEmits<{
  (e: 'action', action: DockerContainerAction): void
}>()

const isProject = (data: ContainerStateData): boolean => 'project' in data

const menuRef = ref<InstanceType<typeof Menu> | null>(null)
const mainAction = computed<DockerContainerAction>(() => (props.data.state !== 'running' ? 'start' : 'stop'))

const projectItems = computed<MenuItem[] | undefined>(() =>
  isProject(props.data)
    ? [
        { label: 'Up', icon: 'pi pi-angle-double-up' },
        { label: 'Down', icon: 'pi pi-angle-double-down' },
        { separator: true },
      ]
    : undefined,
)

const items = computed<MenuItem[]>(() => [
  ...(projectItems.value ?? []),
  { label: 'Start', icon: 'pi pi-play', command: () => emit('action', 'start') },
  { label: 'Stop', icon: 'pi pi-stop', command: () => emit('action', 'stop') },
  { label: 'Restart', icon: 'pi pi-refresh', command: () => emit('action', 'restart') },
  { label: 'Pause', icon: 'pi pi-pause', command: () => emit('action', 'pause') },
  { label: 'Remove', icon: 'pi pi-trash', command: () => emit('action', 'remove') },
])

const toggleMenu = (event: Event) => menuRef.value?.toggle(event)
</script>

<template>
  <ButtonGroup>
    <Button
      v-tooltip.top="mainAction"
      :icon="mainAction === 'start' ? 'pi pi-play' : 'pi pi-stop'"
      size="small"
      @click="emit('action', mainAction)"
    />
    <Button icon="pi pi-ellipsis-v" size="small" @click="toggleMenu" />
  </ButtonGroup>
  <Menu ref="menuRef" :model="items" :popup="true" />
</template>
