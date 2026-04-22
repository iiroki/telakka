# Telakka

Simple Docker management GUI on top of the Docker CLI.

_Tech stack:_

- _Backend:_ Rust, Tauri — connects to the Docker CLI.
- _Frontend:_ TypeScript, Vue 3, Vite, PrimeVue, Pinia.

## Features

- TODO

## Background

With the Docker Desktop requiring a commercial license for large businesses,
there's a gap for a simple and lightweight Docker GUI with good Docker Compose support.

Most of the already existing solutions want to "own and control" Docker projects,
which does not fit the workflow of cloning a repo with an already existing `docker-compose.yml` and
running the project from there.
One feature that was also generally missing was the ability to view and manage all of
the Docker Compose projects from a single interface.

What does a developer do in this case? Build their own solution, of course!

## create-tauri-app

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
