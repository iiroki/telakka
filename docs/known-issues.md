# Known issues

Places where the current app doesn't match the [feature docs](./features/) or [ADRs](./adr/). Planned but unbuilt behavior is marked `(TODO)` in the feature docs instead.

## Docker

- **Project Up/Down do nothing:** the menu items exist but aren't wired to any action.
- **Project menu offers Remove:** Projects have no Remove action in the backend, so it fails. Project rows should only offer Compose actions.
- **Project Up would hang:** it runs Compose without detaching, and without the Project's compose files or working directory.
- **Plain container actions don't separate options from the container ID:** the command doesn't end option parsing before the ID.
- **Removing a running non-project container fails:** project containers are removed with stop and force, but plain containers are removed without force, which Docker refuses for running containers.
- **Logs route shows the wrong container:** the logs route passes the Tab's ID instead of the container ID. Nothing navigates to it yet.

## Backend

- **Receipt includes volatile fields:** the Receipt used for merging covers every field, including the status text, so items change on every status update. A second, durable digest without volatile fields exists but is only used to cache table rows. This contradicts [ADR 0007](./adr/0007-receipt-based-merge.md).
- **Sort orders can disagree:** the backend sorts containers and Projects by lowercased name, but the frontend merge compares names with a locale-aware comparison. The two orders can differ for some names, which breaks the merge's sorted-list invariant. Projects are also collected into an unordered map and only sorted afterwards; they should be built in sorted order, using the same comparison as the frontend merge.

## Layout

- **Status dialog shows "server unreachable" in the wrong case:** it appears when Compose is missing, not when the server is unreachable.
- **Search shortcut fires in every Tab:** inactive Tabs stay mounted, so every Containers Tab registers `Mod` + `F`, and focus can land in a hidden Tab's search box.

## Settings

- **Unused old container list conflicts with View preferences:** a leftover, unused container list component stores its column widths under the same key in an incompatible format. If that data is ever written, the container list shows a "Failed to load UI setting" warning on load. The component should be deleted.
