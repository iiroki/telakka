# 0002 Stateless backend over the Docker CLI

The Tauri backend talks to Docker only by running the Docker CLI, and it holds no Docker state: every call returns fresh data or runs a single action.
The frontend owns all state and coordinates the backend's commands.

Using the CLI instead of the Docker Engine API means the app behaves exactly like the user's terminal: the same Docker context, configuration and Compose plugin, with no separate connection to set up.
Keeping the backend stateless keeps it a thin data source and puts all state logic in one place.
