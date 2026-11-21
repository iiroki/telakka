# Backend

The Backend serves Docker state to the UI. It has two parts:

- **Tauri backend:** a stateless bridge that runs Docker CLI commands and reads/writes Settings (see [ADR 0002](../adr/0002-stateless-backend-over-docker-cli.md)).
- **Frontend backend layer:** polls the Tauri backend, keeps the latest Docker state, and runs actions.

## Commands

| Command            | Returns / does                                                                                       |
| ------------------ | ----------------------------------------------------------------------------------------------------- |
| Status             | Docker client info and context, server info (when reachable) and Compose version (when available). Client info is returned even when the server is down. |
| Containers         | All containers, running or not, sorted by name, plus the Projects derived from them.                   |
| Container stats    | One snapshot of CPU % and memory % for each running container.                                       |
| Container details  | Full inspect details for one container.                                                              |
| Container action   | Runs an action on one container (see [Container actions](./docker.md#container-actions)).            |
| Project action     | Runs an action on one Project (see [Project actions](./docker.md#project-actions)).                  |
| Settings           | Reads Settings (reporting any sections that fell back to defaults), writes sections, and returns the settings directory (see [Settings](./settings.md)). |

### Containers

For each container the backend returns its ID, name, image, platform, state, status text, labels and ports.
Containers that belong to a Project also carry their Project name, Service name and compose file paths, taken from the compose labels.

### Projects

Projects are derived from the containers' compose labels (see [ADR 0001](../adr/0001-projects-are-discovered-not-owned.md)). A Project's state is:

| State   | When                                             |
| ------- | ------------------------------------------------ |
| Running | Every container is running (or restarting).      |
| Partial | Some, but not all, containers are running.       |
| Unknown | Every container's state is unknown.              |
| Exited  | Otherwise.                                       |

## Pollers

Docker state is kept fresh by independent Pollers (see [ADR 0004](../adr/0004-polling-over-events.md)).

| Poller     | Interval                                                                                  |
| ---------- | ----------------------------------------------------------------------------------------- |
| Status     | Polling interval (default 1 s)                                                            |
| Containers | Polling interval (default 1 s)                                                            |
| Stats      | Continuous: the next fetch starts when the previous one finishes, because the stats call itself takes a while. Waits 10 s after an error. |

- (TODO) The polling interval comes from [Settings](./settings.md#backend-todo); it is currently fixed at 1 s.
- A Poller never starts a new fetch while its previous one is still running.
- Each Poller has its own health: **initializing** until its first result, then **healthy** or **errored**. Health changes are logged once per change, not on every poll.
- Pollers start when the app starts and stop when it closes.

## Merging polled state

Polled lists are merged into the current state by Receipt, so unchanged items keep their identity and an unchanged list stays the same list (see [ADR 0007](../adr/0007-receipt-based-merge.md)).

- Containers have a Receipt.
- Projects are compared by state.
- Stats are replaced on every poll.

## Actions

Actions mark the item busy until the first poll after the command returns (see [ADR 0005](../adr/0005-actions-do-not-wait-for-results.md)).

- (TODO) Busy state: the item shows a busy indicator and its actions are disabled while busy.
- If the command fails, the busy state ends and an error notification is shown.

## Logging

- Backend commands are logged at info level.
- The log level is set with the `TELAKKA_LOG_LEVEL` environment variable (default: info).
