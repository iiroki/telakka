import { defineStore } from 'pinia'
import { THEME_ACCENT_COLORS, THEME_MODES, updateThemeAccentColor, updateThemeMode } from '../theme'
import { DeepReadonly, ref } from 'vue'
import { useNotificationStore } from './notification'
import { storage } from '../utils/storage'
import z from 'zod/v4'

/**
 * Every field is defaulted, so the schema is the single source of the default settings
 * and partially persisted settings are filled in on load.
 */
export const zSettings = z.object({
  theme: z
    .object({
      mode: z.literal(THEME_MODES).default('system'),
      accentColor: z.literal(THEME_ACCENT_COLORS).default('indigo'),
    })
    .prefault({}),
  containers: z
    .object({
      groupByProject: z.boolean().default(true),
    })
    .prefault({}),
})

export type Settings = DeepReadonly<z.infer<typeof zSettings>>

const SETTINGS_KEY = 'settings'

export const useSettingsStore = defineStore('settings', () => {
  const { notify } = useNotificationStore()
  const settings = ref<Settings>(zSettings.parse({}))

  const loadSettings = async () => {
    try {
      const s = await storage.read(SETTINGS_KEY, zSettings)
      if (s) {
        settings.value = s
      }
    } catch (err) {
      notify({ level: 'warn', title: 'Failed to load settings', err })
    }
  }

  const saveSettings = async (s: Settings) => {
    settings.value = s
    applySettings()
    notify({ level: 'success', title: 'Settings saved' })
    try {
      await storage.write(SETTINGS_KEY, settings.value)
    } catch (err) {
      notify({ level: 'warn', title: 'Failed to persist settings', err })
    }
  }

  const applySettings = async (load = false) => {
    if (load) {
      await loadSettings()
    }

    updateThemeMode(settings.value.theme.mode)
    updateThemeAccentColor(settings.value.theme.accentColor)
  }

  return { settings, loadSettings, saveSettings, applySettings }
})
