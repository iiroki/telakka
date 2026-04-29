# ADR / State

- Settings — a JSON file handled by the Tauri backend.
- TODO: Tab states

## Frontend-Backend

Frontend uses polling to fetch the Tauri backend state, which communicates directly with the Docker CLI.
This means that the backend is stateless and acts only as a data source.
The frontend is reponsible for managing the state and orchestrating backend commands.

## Container management

App polls the Tauri backend for the list of containers.
When managing containers and invoking commands, the frontend sets its internal state to "loading" (spinned) and
waits for the next backend polling reponse to update the proper state.

## Receipt-based merge

Polled list resources carry a `receipt` (hash digest) computed by the backend over identity-relevant fields only.
Volatile fields (timestamps, transient status detail) are excluded so identical states produce identical receipts.

The frontend merges incoming polled data against current state using a two-pointer sorted merge,
preserving object identity for items whose receipt is unchanged.
When nothing has changed, the merge returns the original array reference — Vue reactivity short-circuits,
downstream computeds skip, and DOM-bound directives (tooltips, popovers) keep their state across polls.

For trivially small resources, fields can be compared directly without an explicit receipt.

**Backend invariant:** polled lists are returned sorted by their identifying field, ascending.
