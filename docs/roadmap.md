# Roadmap

Features that aren't designed yet. Once a feature is designed, it moves into the [feature docs](./features/) and is marked `(TODO)` there until built.
Incomplete parts of existing features are marked `(TODO)` in the feature docs directly.

## Docker resources

- **Images:** list and manage images.
- **Volumes:** list and manage volumes.
- **Networks:** list and manage networks.
- **Container details:** a detailed view of one container with its information and actions. Currently only shows the name, ID, image and raw inspect data.
- **Container logs:** a log stream for one container.
- **Project details:** a detailed view of one Project and its containers.
- **Re-running a project that was taken down:** e.g. starting a project from a compose file, since a Project disappears after Down (see [ADR 0001](./adr/0001-projects-are-discovered-not-owned.md)).

## App

- **Notification history:** the Notifications tab, listing past notifications.
- **Resource monitor:** the Resource monitor tab, with resource usage graphs, including the raw CPU % (which can exceed 100 %).
