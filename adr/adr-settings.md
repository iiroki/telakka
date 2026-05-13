# ADR / Settings

Settings are persisted using the backend's storage system,
which enables file-based persistence.

## Theme

**Mode:** `system` | `light` | `dark`

- `system` — follows OS preference.
- `light` / `dark` — explicit override, toggled via `.theme-mode-dark` class on `<html>`

**Accent color:** one of PrimeVue's supported colors (e.g. `indigo`, `blue`, `red`, etc.)

- Maps to palette shades `0, 50, 100, ..., 900, 950`
- Applied at runtime via PrimeVue `updatePrimaryPalette`
- Default: `indigo`

**Persistence:** settings stored via Pinia — applied on initialization and on every save.

**Schema:** validated with Zod.
