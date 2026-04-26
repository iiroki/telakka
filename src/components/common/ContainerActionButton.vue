<script setup lang="ts">
import { ButtonGroup, Button, Menu } from 'primevue'
import { computed, ref } from 'vue'
import { DockerComposeProject, DockerContainer, DockerContainerCommand } from '../../tauri/bindings.gen'

const props = defineProps<{
  readonly data: DockerContainer | DockerComposeProject
}>()

const emit = defineEmits<{
  (e: 'action', action: DockerContainerCommand): void
}>()

const menuRef = ref<InstanceType<typeof Menu> | null>(null)

const mainAction = computed<DockerContainerCommand>(() => (props.data.state !== 'running' ? 'start' : 'stop'))

const items = [
  { label: 'Start', icon: 'pi pi-play', command: () => emit('action', 'start') },
  { label: 'Stop', icon: 'pi pi-stop', command: () => emit('action', 'stop') },
  { label: 'Restart', icon: 'pi pi-refresh', command: () => emit('action', 'restart') },
  { label: 'Pause', icon: 'pi pi-pause', command: () => emit('action', 'pause') },
  { label: 'Remove', icon: 'pi pi-trash', command: () => emit('action', 'remove') },
]

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
