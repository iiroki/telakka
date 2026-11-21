# Docker

Docker resources shown in Workspace tabs. Only containers exist today; images, volumes and networks are on the [roadmap](../roadmap.md).

## Container list

The Containers view: a table of every container, running or not, with grouping and actions. It has the default [table behavior](./tables.md).

### States

Each container and Project shows a colored state icon.

| State      | Applies to          | Color  |
| ---------- | ------------------- | ------ |
| Running    | Container, Project  | Green  |
| Partial    | Project             | Orange |
| Created    | Container           | Yellow |
| Restarting | Container           | Yellow |
| Paused     | Container           | Blue   |
| Exited     | Container, Project  | Red    |
| Dead       | Container           | Dark grey |
| Removing   | Container           | Dark grey |
| Unknown    | Container, Project  | Grey   |

### Grouping

Controlled by the **Group by project** [setting](./settings.md#containers).

- **On:**
  - Containers without a Project are listed first, without a group header.
  - Each Project follows as a collapsible group, with a header row showing the Project's state, name and [Project actions](#project-actions).
  - Project groups start collapsed. Expanded/collapsed state isn't remembered.
  - Containers in a group are shown by their Service name.
  - Selecting a Project's name opens Containers › Project in the current Tab.
  - Searching filters each group's containers. A group is shown only while at least one of its containers matches.
- **Off:** a flat list where every container shows its full name.

### Columns

| Column   | Shows                                              | Visible by default |
| -------- | -------------------------------------------------- | ------------------ |
| Name     | State icon and name. Locked.                       | Yes                |
| ID       | Container ID                                       | No                 |
| Image    | Image name                                         | Yes                |
| Platform | Image platform                                     | No                 |
| Ports    | Host-bound ports as `host:container`, one per line | Yes                |

### Options

| Option       | Default |
| ------------ | ------- |
| Show actions | Yes     |

### Search fields

- name
- image
- ports, as `host:container`

### Selecting a container

Selecting a container row opens Containers › Container.

## Container actions

Shown per row when the actions column is visible: a main button plus a menu that always lists every action; actions that don't fit the current state are disabled, not hidden.

| Action  | Enabled when                        |
| ------- | ----------------------------------- |
| Start   | Neither running, restarting nor paused |
| Stop    | Running, restarting or paused       |
| Restart | Not restarting                      |
| Pause   | Running                             |
| Unpause | Paused                              |
| Remove  | Always (asks for confirmation)      |

The main button changes with the state:

| State      | Main button |
| ---------- | ----------- |
| Running    | Stop        |
| Restarting | Stop        |
| Paused     | Unpause     |
| Otherwise  | Start       |

- (TODO) Unpause, state-based enabling and the Remove confirmation.
- When a container belongs to a Project, its actions run through Compose on its Service (see [ADR 0006](../adr/0006-project-container-actions-go-through-compose.md)).
- Running an action shows an info notification and marks the container busy (see [Backend › Actions](./backend.md#actions)).

## Project actions

Shown on Project group header rows when the actions column is visible, like container actions. Project rows only offer Docker Compose actions on the whole Project.

| Action  | Does                                                                         |
| ------- | ---------------------------------------------------------------------------- |
| Up      | Creates and starts the Project from its compose files, detached.             |
| Down    | Stops and removes the Project's containers and networks. Asks for confirmation. |
| Start   | Starts the Project's existing containers.                                    |
| Stop    | Stops the Project's containers.                                              |
| Restart | Restarts the Project's containers.                                           |
| Pause   | Pauses the Project's containers.                                             |
| Unpause | Unpauses the Project's paused containers.                                    |

- (TODO) Up, Down and Unpause from the menu, and the Down confirmation.
- Running an action shows an info notification and marks the Project busy (see [Backend › Actions](./backend.md#actions)).
- After Down, the Project's containers no longer exist, so the Project disappears from the list (see [ADR 0001](../adr/0001-projects-are-discovered-not-owned.md)).

## Confirmations

Destructive actions (container Remove, Project Down) always ask for confirmation before running.
