# ADR / Settings

Settings are persisted using the backend's storage system,
which enables file-based persistence.

## Theme

**Mode:** `system` | `dark` | `light`

- `system` — follows OS preference via PrimeVue `darkModeSelector: 'system'`
- `dark` / `light` — explicit override, toggled via `.dark-mode` class on `<html>`

**Accent color:** one of PrimeVue's supported colors (e.g. `indigo`, `blue`, `red`, etc.)

- Maps to palette shades `0, 50, 100, ..., 900, 950`
- Applied at runtime via PrimeVue `updatePrimaryPalette`
- Default: `indigo`

**Persistence:** settings stored via Pinia (`useSettingsStore`). Applied on load and on every save.

**Schema:** validated with Zod (`zSettings` in `src/stores/settings.ts`).
