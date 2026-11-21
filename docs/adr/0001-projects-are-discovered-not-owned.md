# 0001 Projects are discovered, never owned

Telakka discovers Docker Compose Projects from the compose labels of existing containers instead of keeping its own list of projects.
Developers already have projects on disk (a cloned repo with a compose file) and run them from there;
tools that require importing or "owning" a project don't fit that workflow, and seeing every project on the machine from one place was the main gap this app fills.

## Consequences

- The app stores nothing about projects. A project is visible only while at least one of its containers exists.
- After a project is taken down, it disappears and can't be brought up again from the app until another way to start a project exists (see the [roadmap](../roadmap.md)).
