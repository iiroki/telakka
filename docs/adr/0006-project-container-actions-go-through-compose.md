# 0006 Project container actions go through Compose

Actions on a container that belongs to a Project are run as Docker Compose commands on that container's Service, not as plain Docker commands on the container.

Compose then stays aware of what happened to its own containers, and the action behaves the same as running it with Compose from the terminal.
Containers that don't belong to a project use plain Docker commands.
