# Concentric Time

A simple and fun way to view your clock!

---

### Dev

This project uses [Dioxus](https://dioxuslabs.com/) (a Rust-Wasm framework) and does also require some Node to run the [Tailwind](https://tailwindcss.com/) CLI (CSS framework) to build the stylesheet.

In one terminal run `pnpm run tw:watch` to start the Tailwind CLI for styles. In another terminal run `trunk serve` which bundles the assets and serves at [localhost:8080](http://localhost:8080).

### Release

First build the CSS with `pnpm run tw:build`. Then build the app with `trunk build --release`. Will build locally and push web assets to hosting provider for simplicity of build tooling.
