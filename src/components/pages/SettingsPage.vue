<script setup lang="ts">
import { Button, FloatLabel, Select } from 'primevue'
import { THEME_ACCENT_COLORS, THEME_MODES } from '../../theme'
import { useSettingsStore, zSettings } from '../../stores/settings'
import { Form, FormSubmitEvent } from '@primevue/forms'
import { reactive } from 'vue'
import { expandObject, flattenObject } from '../../utils/obj'
import Dot from '../common/Dot.vue'

// Take a copy of the settings, modify it, and save when ready.
const { settings, saveSettings } = useSettingsStore()
const init = reactive(flattenObject(settings))
const onSubmit = (e: FormSubmitEvent<Record<string, unknown>>) => {
  const parsed = zSettings.safeParse(expandObject(e.values))
  if (parsed.success) {
    saveSettings(parsed.data)
  }
}
</script>

<template>
  <h2>Settings</h2>
  <Form :initial-values="init" style="display: flex; flex-direction: column; gap: 1rem" @submit="onSubmit">
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

    <h3>Backend</h3>
    <span style="font-style: italic">TODO</span>

    <Button type="submit" style="max-width: 120px; margin-top: 1rem"> Save </Button>
  </Form>
</template>
