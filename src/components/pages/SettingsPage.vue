<script setup lang="ts">
import { Button, FloatLabel, Select } from 'primevue'
import { THEME_ACCENT_COLORS, THEME_MODES } from '../../theme'
import { useSettingsStore, zSettings } from '../../stores/settings'
import { Form, FormSubmitEvent } from '@primevue/forms'
import { reactive, ref } from 'vue'
import { revealItemInDir } from '@tauri-apps/plugin-opener'
import { commands } from '../../tauri/bindings.gen'
import { expandObject, flattenObject } from '../../utils/obj'
import Dot from '../common/Dot.vue'
import LabelCheckbox from '../common/LabelCheckbox.vue'

// Take a copy of the settings, modify it, and save when ready.
const { settings, saveSettings } = useSettingsStore()
const init = reactive(flattenObject(settings))
const dir = ref<string | null>(null)

const onSubmit = (e: FormSubmitEvent<Record<string, unknown>>) => {
  const parsed = zSettings.safeParse(expandObject(e.values))
  if (parsed.success) {
    saveSettings(parsed.data)
  }
}

const openDir = () => {
  if (dir.value) {
    revealItemInDir(dir.value)
  }
}

commands.storageGetDir().then((r) => (dir.value = r.status === 'ok' ? r.data : null))
</script>

<template>
  <h2>Settings</h2>

  <div class="settings-dir" style="margin-bottom: 0.5rem">
    <span>{{ dir ?? '???' }}</span>
    <Button
      v-tooltip.top="'Open'"
      icon="pi pi-folder-open"
      text
      rounded
      size="small"
      severity="secondary"
      @click="openDir"
    />
  </div>

  <Form :initial-values="init" style="display: flex; flex-direction: column; gap: 2rem" @submit="onSubmit">
    <div class="settings-block">
      <h3>Theme</h3>
      <FloatLabel variant="on">
        <Select name="theme.mode" label-id="theme-mode" :options="[...THEME_MODES]" style="width: 160px" />
        <label for="theme-mode">Mode</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <Select
          name="theme.accentColor"
          label-id="theme-accent-color"
          :options="[...THEME_ACCENT_COLORS]"
          style="width: 160px"
        >
          <template #value="slot">
            <Dot :color="`var(--p-${slot.value}-500)`" size="0.6rem" style="margin-right: 0.25rem" />
            {{ slot.value }}
          </template>
          <template #option="slot">
            <Dot :color="`var(--p-${slot.option}-500)`" size="0.6rem" style="margin-right: 0.5rem" />
            {{ slot.option }}
          </template>
        </Select>
        <label for="theme-accent-color">Accent color</label>
      </FloatLabel>
    </div>

    <div class="settings-block">
      <h3>Backend</h3>
      <span style="font-style: italic">TODO</span>
    </div>

    <div class="settings-block">
      <h3>Containers</h3>
      <LabelCheckbox name="containers.groupByProject" label="Group by project" />
    </div>

    <Button type="submit" style="max-width: 120px; margin-top: 1rem"> Save </Button>
  </Form>
</template>

<style scoped>
.settings-dir {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: monospace;
  font-size: 0.875rem;
  color: var(--p-text-muted-color);
}

.settings-block {
  display: flex;
  flex-direction: column;
  align-items: start;
  gap: 1rem;
}

/* Spacing comes from the block gap only. */
.settings-block h3 {
  margin: 0;
}
</style>
