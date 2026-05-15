# ADR / State

Frontend uses the Tauri backend to communicate directly with the Docker CLI.
This means that the backend is stateless and acts only as a data source.
The frontend is responsible for managing the state and orchestrating backend commands.

## Stores

State is managed by Pinia stores.

### Settings

Settings store is responsible for managing the app settings in frontend and synchronizing them with the backend,
which is responsible for persisting them to the disk.

Settings are validated with Zod on load to ensure the app is always working with a valid configuration.
All settings changes are applied immediately on save.

If the app receives invalid settings from the backend,
the default settings shall be used instead and a warning should be logged.

### Tabs

Tab store is responsible for managing the multiple tabs that can be open in the app at the same time.

Tabs are separated into two categories:

- **Workspace:** A regular workspace tab that contains the typical Docker management capabilities,
  such as containers, images, volumes, etc.
  These tabs are the main focus of the app and are designed to be used in a multi-tab workflow,
  allowing users to manage multiple aspects of their Docker environment simultaneously.
  - Workspace tabs also contain an internal navigation system,
    allowing users to navigate between different views (containers, images, volumes, etc.) within the same tab.
- **Special:** A special standalone tab, such as settings, notifications, etc.
  - Opening a special tab that already exists focuses the existing one instead of creating a duplicate.

### Notifications

Notification store dispatches notifications to the PrimeVue toast UI and the internal logger simultaneously.
Default toast duration: 3 s for info/success, 10 s for warn/error.

### Backend

Backend store is responsible for serving the Docker backend state to the frontend,
such as containers, images, volumes, etc.
Exposes helpers for running container and project actions.
See ["Backend management"](#backend-management) below for more details.

### Backend management

Backend management store is responsible for managing the backend state,
which means fetching the up-to-date state and populating the ["Backend"](#backend) store.

Runs independent pollers for status, containers, and stats. The polling interval is configurable.
Each poller tracks its own health state: initializing, healthy, or errored.

#### Container management

App polls the Tauri backend for the list of containers.
When managing containers and invoking commands, the frontend sets its internal state to "loading" (spinned) and
waits for the next backend polling response to update the proper state.

#### Receipt-based merge

Polled list resources carry a `receipt` (hash digest) computed by the backend over identity-relevant fields only.
Volatile fields (timestamps, transient status detail) are excluded so identical states produce identical receipts.

The frontend merges incoming polled data against current state using a two-pointer sorted merge,
preserving object identity for items whose receipt is unchanged.
When nothing has changed, the merge returns the original array reference — Vue reactivity short-circuits,
downstream computeds skip, and DOM-bound directives (tooltips, popovers) keep their state across polls.

For trivially small resources, fields can be compared directly without an explicit receipt.

**Backend invariant:** polled lists are returned sorted by their identifying field, ascending.
