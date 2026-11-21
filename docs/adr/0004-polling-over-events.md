# 0004 Polling over events

The frontend keeps Docker state up to date by polling the backend with independent Pollers (status, containers, stats) instead of subscribing to Docker events.

Polling fits the stateless backend: every poll returns the full, current state, so missed events, reconnects and event-to-state reconstruction never need handling.
Each Poller runs and fails independently, so a slow stats call never delays the container list, and Docker being unavailable shows up as a Poller error instead of a broken stream.
