import Aura from '@primeuix/themes/aura'
import { definePreset, updatePrimaryPalette } from '@primeuix/themes'
import { PaletteDesignToken } from '@primeuix/themes/types'

export const THEME_MODES = ['system', 'light', 'dark'] as const
export type ThemeMode = (typeof THEME_MODES)[number]

export const THEME_ACCENT_COLORS = [
  'emerald',
  'green',
  'lime',
  'red',
  'orange',
  'amber',
  'yellow',
  'teal',
  'cyan',
  'sky',
  'blue',
  'indigo',
  'violet',
  'purple',
  'fuchsia',
  'pink',
  'rose',
] as const
export type ThemeAccentColor = (typeof THEME_ACCENT_COLORS)[number]

export const updateThemeAccentColor = (accent: ThemeAccentColor) =>
  updatePrimaryPalette(
    Object.fromEntries(
      ([0, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950] satisfies (keyof PaletteDesignToken)[]).map((s) => [
        s,
        `{${accent}.${s}}`,
      ]),
    ),
  )

/** Custom class used to indicate dark mode. */
export const DARK_MODE_CLASS = 'theme-mode-dark'

// Single instance required — add/remove must reference the same MediaQueryList object.
const osQuery = window.matchMedia('(prefers-color-scheme: dark)')
let osListener: (() => void) | null = null

/**
 * Toggles {@link DARK_MODE_CLASS} on `<html>`.
 *
 * In `system` mode, syncs with OS preference via a matchMedia listener
 * that is cleaned up when switching to an explicit mode.
 */
export const updateThemeMode = (mode: ThemeMode) => {
  if (osListener) {
    osQuery.removeEventListener('change', osListener)
    osListener = null
  }

  if (mode === 'system') {
    const sync = () => document.documentElement.classList.toggle(DARK_MODE_CLASS, osQuery.matches)
    sync()
    osListener = sync
    osQuery.addEventListener('change', osListener)
  } else {
    document.documentElement.classList.toggle(DARK_MODE_CLASS, mode === 'dark')
  }
}

/**
 * Telakka theme, based on PrimeVue Aura with few adjustments.
 */
const Theme = definePreset(Aura, {
  css: () => `
    :root {
      --nav-background: color-mix(in srgb, var(--p-content-background) 95%, var(--p-gray-400));
      border-top: 1px solid color-mix(in srgb, var(--p-content-background) 90%, var(--p-gray-900));
    }
    .${DARK_MODE_CLASS} {
      --nav-background: color-mix(in srgb, var(--p-content-background) 80%, black);
      border-top: 0;
      padding-top: 1px;
    }
  `,
})

export default Theme
