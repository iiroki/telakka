<script setup lang="ts">
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { commands, type DockerStatus } from '../gen/tauri'
import KeyValueTable from './common/KeyValueTable.vue'

const status = ref<DockerStatus | null>(null)
const error = ref<string | null>(null)
const statusDialogOpen = ref(false)

const POLL_MS = 5000
let timer: ReturnType<typeof setInterval> | null = null

const refreshStatus = async () => {
  const res = await commands.status()
  if (res.status === 'ok') {
    status.value = res.data
    error.value = null
  } else {
    status.value = null
    error.value = res.error
  }
}

const statusColor = computed(() => {
  if (!status.value) return 'var(--p-red-500)'
  if (!status.value.server) return 'var(--p-yellow-500)'
  return 'var(--p-green-500)'
})

const label = computed(() =>
  status.value ? `Docker ${status.value.server?.version ?? '???'} (${status.value.client.context})` : 'Docker down',
)

onMounted(() => {
  refreshStatus()
  timer = setInterval(refreshStatus, POLL_MS)
})

onUnmounted(() => {
  if (timer) {
    clearInterval(timer)
  }
})
</script>

<template>
  <div class="bottom-bar">
    <Button class="status-btn" text size="small" @click="statusDialogOpen = true">
      <span class="dot" :style="{ background: statusColor }" />
      <span>{{ label }}</span>
    </Button>

    <Button :icon="'pi pi-cog'" text size="small" aria-label="Settings" :style="{ color: 'var(--p-text-color)' }" />
  </div>

  <Dialog v-model:visible="statusDialogOpen" modal header="Docker | Status" :style="{ width: '30rem' }">
    <div v-if="status" class="details">
      <!-- Client -->
      <section>
        <h3>Client</h3>
        <h4 :style="{ fontStyle: 'italic' }">{{ status.client.name }}</h4>
        <KeyValueTable
          :value-style="{ fontFamily: 'monospace' }"
          :rows="[
            { label: 'Version', value: status.client.version },
            { label: 'API version', value: status.client.apiVersion },
            { label: 'OS', value: status.client.os },
            { label: 'Arch', value: status.client.arch },
            { label: 'Context', value: status.client.context },
          ]"
        />
      </section>
      <!-- Server -->
      <section v-if="status.server">
        <h3>Server</h3>
        <h4 :style="{ fontStyle: 'italic' }">{{ status.server.name }}</h4>
        <KeyValueTable
          :value-style="{ fontFamily: 'monospace' }"
          :rows="[
            { label: 'Version', value: status.server.version },
            { label: 'API version', value: status.server.apiVersion },
            { label: 'OS', value: status.server.os },
            { label: 'Arch', value: status.server.arch },
          ]"
        />
      </section>
      <section v-else class="error">
        <h4>Server</h4>
        <div>Daemon unreachable.</div>
      </section>
    </div>
    <div v-else class="details error">
      <div>Docker unavailable.</div>
      <pre v-if="error">{{ error }}</pre>
    </div>
  </Dialog>
</template>

<style scoped>
.bottom-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.25rem 0.75rem;
  border-top: 1px solid var(--p-content-border-color);
  background: var(--p-content-background);
  font-size: 0.85rem;
}

.status-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.dot {
  width: 0.6rem;
  height: 0.6rem;
  border-radius: 50%;
  display: inline-block;
}

.details {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.details section h4 {
  margin: 0 0 0.35rem;
  font-size: 0.9rem;
  opacity: 0.7;
}

.details section > div {
  font-size: 0.85rem;
  line-height: 1.4;
}

.details.error pre {
  white-space: pre-wrap;
  font-size: 0.75rem;
  opacity: 0.8;
}
</style>
