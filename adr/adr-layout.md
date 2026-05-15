# ADR / Layout

## Components

- **Tab bar (top)**
  - Opened tabs — first tab (containers) is created automatically on startup
  - New tab button "+"
- **Main area** — contains the active tab content:
  - **Workspace tabs** — a left sidebar and a main content area:
    - Left sidebar (icon-only menu with tooltips):
      - Containers (active)
      - Images
      - Volumes
      - Networks
    - Main content:
      - Breadcrumb header
      - Page content
  - **Special tabs** — render full-width with no breadcrumb or sidebar, since those are workspace tab features.
- **Bottom bar**
  - Left:
    - Docker status indicator — color-coded dot + version label, opens a status dialog with client/server/compose details.
  - Right:
    - Resource usage (CPU %, Mem %) — opens resource monitor tab.
    - Notification icon — opens notifications tab.
    - Settings icon — opens settings tab.

## Tab routes

> Types:
>
> - **W** = workspace
> - **S** = special

| Route                        | Type | Description                                                   |
| ---------------------------- | ---- | ------------------------------------------------------------- |
| `containers`                 | W    | Container list with navigation and quick actions              |
| `containers/:id` (TODO)      | W    | Detailed container view with detailed information and actions |
| `containers/:id/logs` (TODO) | W    | Container log stream                                          |
| `_settings`                  | S    | Settings management                                           |
| `_notifications` (TODO)      | S    | Notification history                                          |
| `_resource-monitor` (TODO)   | S    | Resource monitor graphs                                       |
