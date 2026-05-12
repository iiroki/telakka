<script setup lang="ts">
import { storeToRefs } from 'pinia'
import Button from 'primevue/button'
import Dialog from 'primevue/dialog'
import { computed, ref } from 'vue'
import KeyValueTable from './common/KeyValueTable.vue'
import { useBackendStore } from '../stores/backend'
import { useTabStore } from '../stores/tab'

const DEFAULT_CONTEXT = 'default'

const statusDialogOpen = ref(false)

const { openOrCreateTab } = useTabStore()
const { status, stats } = storeToRefs(useBackendStore())

const statusColor = computed(() => {
  if (!status.value) return 'var(--p-red-500)'
  if (!status.value.server) return 'var(--p-yellow-500)'
  return 'var(--p-green-500)'
})

const label = computed(() => {
  const parts: (string | null)[] = ['Docker']
  if (status.value) {
    parts.push(
      status.value.server?.version ?? '???',
      status.value.client.context !== DEFAULT_CONTEXT ? `(${status.value.client.context})` : null,
    )
  } else {
    parts.push('unavailable')
  }

  return parts.filter(Boolean).join(' ')
})

const resourceUsage = computed(() => {
  if (!stats.value) {
    return null
  }

  let cpuPerc = 0
  let memPerc = 0
  for (const s of stats.value) {
    cpuPerc += s.cpuPercentage
    memPerc += s.memPercentage
  }

  return { cpuPerc, memPerc }
})
</script>

<template>
  <div class="bottom-bar">
    <Button class="status-btn" text severity="secondary" size="small" @click="statusDialogOpen = true">
      <span class="dot" :style="{ background: statusColor }" />
      <span>{{ label }}</span>
    </Button>

    <div>
      <Button text size="small" severity="secondary" @click="openOrCreateTab({ key: '_resource-monitor' })">
        <span>
          CPU:
          <span style="display: inline-block; width: 6ch; text-align: right">{{
            resourceUsage?.cpuPerc.toFixed(2) ?? '-'
          }}</span>
          %
        </span>
        <span>
          Mem:
          <span style="display: inline-block; width: 5ch">{{ resourceUsage?.memPerc.toFixed(2) ?? '-' }}</span>
          %
        </span>
      </Button>
      <Button
        icon="pi pi-bell"
        text
        severity="secondary"
        size="small"
        aria-label="Notifications"
        @click="openOrCreateTab('_notifications')"
      />
      <Button
        icon="pi pi-cog"
        text
        severity="secondary"
        size="small"
        aria-label="Settings"
        @click="openOrCreateTab({ key: '_settings' })"
      />
    </div>
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
      <section v-if="status.compose">
        <h3>Compose</h3>
        <KeyValueTable
          :value-style="{ fontFamily: 'monospace' }"
          :rows="[{ label: 'Version', value: status.compose ?? 'N/A' }]"
        />
      </section>
      <section v-else class="error">
        <h4>Server</h4>
        <div>Server unreachable.</div>
      </section>
    </div>
    <div v-else class="details error">
      <div>Docker unavailable</div>
    </div>
  </Dialog>
</template>

// TODO: Review this code-generated crap...
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
