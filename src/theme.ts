import Aura from '@primeuix/themes/aura'
import { definePreset, updatePreset, updatePrimaryPalette } from '@primeuix/themes'
import { PaletteDesignToken } from '@primeuix/themes/types'

export const THEME_MODES = ['system', 'dark', 'light'] as const
export type ThemeMode = (typeof THEME_MODES)[number]

export const THEME_ACCENT_COLORS = ['amber', 'emerald', 'fuchsia', 'indigo', 'purple', 'rose', 'sky', 'teal'] as const
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

export const updateThemeMode = (mode: ThemeMode) => {
  if (mode === 'system') {
    document.documentElement.classList.remove('p-dark')
    updatePreset({ darkModeSelector: 'system' })
  } else {
    updatePreset({ darkModeSelector: '.p-dark' })
    document.documentElement.classList.toggle('p-dark', mode === 'dark')
  }
}

const Theme = definePreset(Aura, {
  // theme: {
  //   options: {
  //     darkModeSelector: 'system'
  //   },
  // },
})

export default Theme
