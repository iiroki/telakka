# Layout

The app window has three areas: the **tab bar** at the top, the **main area** showing the active Tab, and the **bottom bar**.

## Tab bar

- Shows the open Tabs. A Workspace tab showing the Containers view is created on startup.
- Each Tab header shows an icon and a label for the Tab's current Route; a long label is truncated and the full label is shown in a tooltip.
- Each Tab has a close button, shown on hover.
- A **"+"** button stays at the right edge and opens a new Workspace tab showing the Containers view.
  - (TODO) At most 100 Tabs can be open; "+" is disabled at the limit.

### Closing tabs

When the active Tab is closed:

- If it was the first Tab, the next Tab becomes active.
- Otherwise, the previous Tab becomes active.
- If it was the only Tab, a new Workspace tab showing the Containers view is created, so there is always at least one Tab.

Closing an inactive Tab doesn't change the active Tab.

### Opening tabs

- Opening a Special tab that is already open makes the existing one active instead of creating a duplicate.
- Workspace tabs are never reused this way: navigating always happens in the current Tab, even if another Tab already shows the same Route.

## Workspace tabs

A Workspace tab has a **sidebar** and a **content area**.

### Sidebar

An icon-only menu, each item with a tooltip. Selecting an item replaces the Tab's Route with that view.

- Containers
- Images (disabled, see the [roadmap](../roadmap.md))
- Volumes (disabled)
- Networks (disabled)

### Content area

- **Breadcrumb** on top, one item per Route segment. Selecting an item goes back to that point in the Route. Detail segments show the resource's name (e.g. the container name).
- **Page content** for the current Route below it.

## Special tabs

Special tabs fill the whole main area, with no sidebar or breadcrumb.

- Settings
- Notifications (see the [roadmap](../roadmap.md))
- Resource monitor (see the [roadmap](../roadmap.md))

## Routes

| Route                   | Tab       | Shows                                                    |
| ----------------------- | --------- | -------------------------------------------------------- |
| Containers              | Workspace | [Container list](./docker.md#container-list)             |
| Containers › Container  | Workspace | Container details (see the [roadmap](../roadmap.md))     |
| Containers › Project    | Workspace | Project details (see the [roadmap](../roadmap.md))       |
| Settings                | Special   | [Settings](./settings.md)                                |
| Notifications           | Special   | Notification history (see the [roadmap](../roadmap.md))  |
| Resource monitor        | Special   | Resource graphs (see the [roadmap](../roadmap.md))       |

## Bottom bar

### Left: Docker status

A status button showing a colored dot and a label.

| State                                   | Dot    | Label                         |
| --------------------------------------- | ------ | ----------------------------- |
| Status not loaded yet                   | Grey   | `...`                         |
| Docker unavailable                      | Red    | `Docker unavailable`          |
| Docker CLI found, server not reachable  | Yellow | `Docker (unreachable)`        |
| OK                                      | Green  | `Docker <server version>`     |

If the active Docker context isn't `default`, the context name is appended in parentheses.

Selecting the button opens a **status dialog** with three sections:

- **Client:** Docker CLI version details and the active context.
- **Server:** Docker server version details, or a "server unreachable" message when the server isn't reachable.
- **Compose:** Docker Compose version, or a message when Compose isn't available.

### Right

- **Resource usage:** total CPU % and memory % across all containers, `-` until stats have loaded. Selecting it opens the Resource monitor tab.
  - Memory % is relative to the machine's total memory.
  - (TODO) CPU % is scaled to 0–100 % of the whole machine (all cores). The raw sum, which can exceed 100 % on multi-core machines, is shown on hover.
- **Notifications** button: opens the Notifications tab.
- **Settings** button: opens the Settings tab.

## Notifications

Every notification is shown as a toast and written to the app log at the same time (unless explicitly not logged).

| Level   | Toast duration |
| ------- | -------------- |
| Info    | 3 s            |
| Success | 3 s            |
| Warn    | 10 s           |
| Error   | 10 s           |

A notification has a title and an optional message. (TODO) When a notification carries an error, the error details are logged.

## Keyboard shortcuts

On macOS, the modifier key is `Cmd`; elsewhere it is `Ctrl`.

| Shortcut       | Where              | Action                |
| -------------- | ------------------ | --------------------- |
| `Mod` + `F`    | Any table          | Focus the table's search box. Applies only to the active Tab. |
