# Plan: Vuetify UI

Rebuild the frontend UI on [Vuetify](https://vuetifyjs.com/), starting with the App Shell.

## Principles

- **No backwards compatibility.** The first version is still in development: prioritize a good new implementation over compatibility with old code, data or Settings.
  Old Settings files are removed, not migrated.
- **Keep it simple.** The smallest change that fully works. No custom abstractions, wrappers or "for later" code.
- **Vuetify first.** Use Vuetify components, layout, theming and defaults instead of implementing our own.
- **Recommended best practices.** Follow Vuetify's officially recommended setup.
- **Dead code goes.** Delete anything that no longer has a use; git history keeps it.

## Phase 1: App Shell

### Dependencies

Installs can't run in the agent sandbox, so **the user runs these** before implementation starts:

```sh
pnpm remove primevue @primeuix/themes @primevue/forms primeicons @fontsource-variable/inter
pnpm add vuetify @mdi/js
pnpm add -D vite-plugin-vuetify
```

### Setup

- Remove every reference to the removed packages, including the license env variable typing in `src/vite-env.d.ts`
  (and its value in `.env.local`, done by the user).
- Use Vuetify's default typography.
- Register Vuetify in `main.ts` and `vite-plugin-vuetify` (auto-import) in `vite.config.ts`.
- Icons: `@mdi/js` (tree-shaken SVG) with Vuetify's `mdi-svg` icon set.
- Global defaults: `density: 'compact'`, set in `createVuetify({ defaults })`.

### Theme

- Light / Dark / System mode via Vuetify themes (`defaultTheme`, `theme.change()`), including Vuetify's built-in `system` theme.
  The custom dark-mode class, `matchMedia` listener and custom CSS variables go away.
- Accent color: a color name from Vuetify's Material palette (`vuetify/util/colors`), stored as a string in Settings; `colors[name].base` becomes the theme's `primary`.
  - Default: `indigo`.
- Settings are read-only until the Settings page is rebuilt, so theme mode and accent can only be changed by editing the settings file.

### Layout

Vuetify's layout system:

- `v-app`
  - `v-app-bar`: tab bar with `v-tabs`. Each Tab shows its icon and label (tooltip on truncation) and a close button on hover. A "+" button at the end opens a new Workspace tab.
  - `v-navigation-drawer` (`rail`): sidebar, shown only when the active Tab is a Workspace tab.
  - `v-main`: `v-breadcrumbs` (Workspace tabs only) and the Tab content, wrapped in `<KeepAlive>` keyed by Tab id.
  - `v-footer` (`app`): bottom bar (Docker status, resource usage, notifications and settings buttons).
- Docker status dialog: `v-dialog`.

Behavior follows [docs/features/layout.md](./docs/features/layout.md).

### Notifications

- The notification store holds a message queue rendered by `v-snackbar-queue` in `App.vue`.
- Levels map to Vuetify colors; toast durations as in [layout.md](./docs/features/layout.md#notifications).

### Pages

Until rebuilt, every page shows its backend store data as raw JSON in a monospace code block:

| Page             | Data                     |
| ---------------- | ------------------------ |
| Container list   | Containers               |
| Container        | The current container    |
| Settings         | Settings (read-only)     |
| Notifications    | None — keeps its current TODO placeholder (no notification history exists yet) |
| Resource monitor | Stats                    |

### Cleanup

- Delete every component, type, style override and utility left unused.

### Docs

- Update the tech stack in `README.md`.
- Update the accent color list in [settings.md](./docs/features/settings.md#theme) to the Material palette names.

### Done when

- `pnpm build` and `pnpm check` pass.
- No references to the removed packages remain in `src/`.
- `pnpm tauri:dev` shows the shell working as described in [layout.md](./docs/features/layout.md) (checked by the user).

## Future work

To be planned later:

- Settings page
- Container list and shared table behavior ([tables.md](./docs/features/tables.md))
- Container page
- Notifications page
- Resource monitor
