# Telakka

Simple Docker management GUI on top of the Docker CLI.

_Tech stack:_

- _Backend:_ Rust, Tauri — connects to the Docker CLI.
- _Frontend:_ TypeScript, Vue 3, Vite, PrimeVue, Pinia.

## Features

- [Containers and Docker Compose projects](./docs/features/docker.md): every container and project on the machine in one list, with search, grouping and actions.
- [Tabs and layout](./docs/features/layout.md): multiple independent tabs, Docker status and resource usage at a glance.
- [Settings](./docs/features/settings.md): theme, container grouping and remembered view preferences.
- [Tables](./docs/features/tables.md): shared column, search and tooltip behavior for every resource table.
- [Backend](./docs/features/backend.md): a stateless bridge to the Docker CLI, kept up to date by polling.

See also: [glossary](./GLOSSARY.md), [architecture decisions](./docs/adr/), [roadmap](./docs/roadmap.md) and [known issues](./docs/known-issues.md).

## Background

With the Docker Desktop requiring a commercial license for large businesses,
there's a gap for a simple and lightweight Docker GUI with good Docker Compose support.

Most of the already existing solutions want to "own and control" Docker projects,
which does not fit the workflow of cloning a repo with an already existing `docker-compose.yml` and
running the project from there.
One feature that was also generally missing was the ability to view and manage all of
the Docker Compose projects from a single interface.

What does a developer do in this case? Build their own solution, of course!
