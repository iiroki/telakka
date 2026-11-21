# Telakka

A desktop GUI for managing a local Docker environment, with first-class support for Docker Compose projects the user already has on disk.

## Docker

**Docker**:
The local Docker installation the app manages (CLI, daemon and Compose), reached through the Docker CLI.
_Avoid_: Engine, daemon, server

**Container**:
A single Docker container known to Docker, running or not.

**Project**:
A Docker Compose project, discovered from the compose labels of its containers. The app never creates or owns projects.
_Avoid_: Stack, compose app, group

**Service**:
A named container definition within a Project.

## Backend

**Backend**:
Everything that serves Docker state to the UI: the Tauri process that talks to the Docker CLI, plus the frontend layer that polls it and holds the latest state.
_Avoid_: Server, API, Docker (when meaning the Tauri process)

**Poller**:
An independent, repeating fetch of one kind of Docker state (status, containers, or stats), with its own health: initializing, healthy, or errored.

**Busy**:
The state of a Container or Project while an action on it is in flight, until the first poll after the action's command returns.

**Receipt**:
A digest of an item's identity-relevant fields, used to tell whether a polled item actually changed.
_Avoid_: Hash, version, etag

## Settings

**Settings**:
The user's persisted app configuration.

**View preferences**:
The part of Settings that remembers UI state for a specific view (e.g. container table columns), edited in that view rather than on the Settings page.
_Avoid_: UI options, table options

## Layout

**Tab**:
One open, independent view in the tab bar.

**Workspace tab**:
A Tab for Docker management, navigable between resource views and details.

**Special tab**:
A standalone, full-width Tab (Settings, Notifications, Resource monitor); at most one of each exists.

**Route**:
The ordered sequence of Route segments describing what a Tab currently shows.
_Avoid_: Path, URL

**Route segment**:
One step of a Route: either a view (e.g. containers) or a detail identifying a specific resource (e.g. one Container).
_Alias_: Segment
