# 0005 Actions don't wait for their result

When the user runs an action (e.g. stopping a container), the item is marked busy and the command is sent; the UI never updates the item from the command's result.
The busy state ends at the first poll after the command returns, and that poll shows the item's real state.
If the command fails, the busy state ends immediately and an error notification is shown.

This keeps Polling as the only source of Docker state, so the UI can never show a state the app guessed. Ending busy on the first poll after the command (not on a state change) is also correct for actions that change nothing, such as restarting or stopping a container that has already exited.
