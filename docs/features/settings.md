# Settings

Settings are stored as a single JSON file in the app's data directory.
The backend defines the schema and defaults, and validates and writes each section separately (see [ADR 0003](../adr/0003-backend-owns-settings-schema.md)).

## Lifecycle

- **Startup:** reading Settings is the first thing the app does. The UI isn't shown until Settings have been read and applied.
- **Invalid sections:** a section that fails validation is replaced with its defaults for this session only; the file on disk isn't changed.
  The user gets a warning notification naming the failed sections, and the warning is logged.
  - (TODO) Currently any invalid value resets all Settings to defaults.
- **Saving:** every change made in the UI is saved with an explicit **Save** button and applied immediately.
  Saving writes only the sections being saved; the rest of the file is left as it is.
  If writing to disk fails, a warning notification is shown.

## Settings page

Opened from the bottom bar's settings button, as a Special tab.
Changes are applied only when **Save** is pressed, which saves every section on the page and shows a "Settings saved" success notification.

The page also shows the settings directory with a button that reveals it in the system file manager.

### Theme

| Setting      | Values                                  | Default  |
| ------------ | --------------------------------------- | -------- |
| Mode         | System, Light, Dark                     | System   |
| Accent color | See below                               | Indigo   |

- **System** follows the OS light/dark preference and updates live when the OS preference changes.
- **Accent color** is one of: emerald, green, lime, red, orange, amber, yellow, teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose.
  It sets the app's primary color in all shades. Each option is shown with a swatch.

### Containers

| Setting          | Values  | Default |
| ---------------- | ------- | ------- |
| Group by project | Yes, No | Yes     |

- **Yes:** project containers are grouped under collapsible Project rows and shown by their Service name (e.g. Project `example-project` › `db`).
- **No:** a flat list where containers show their full name (e.g. `example-project-db`).

### Backend (TODO)

| Setting          | Values           | Default |
| ---------------- | ---------------- | ------- |
| Polling interval | 100 ms or more   | 1 s     |

Applies to the status and container [Pollers](./backend.md#pollers).

## View preferences

View preferences are stored in Settings, in a separate section per view, but aren't shown on the Settings page.
They are edited in the view they belong to and saved there with its own **Save** button.

- Tables: see [Column selector](./tables.md#column-selector).

(TODO) View preferences are currently stored in a separate file and saved with an "Apply" button.
