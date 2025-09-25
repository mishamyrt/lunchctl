# lunchctl

Lightweight Rust library for creating and controlling macOS Launch Agents (launchd) via `launchctl`. It generates plist files in `~/Library/LaunchAgents`, starts/stops agents, and lets you query their state.

## Features

Create plist-backed Launch Agents, bootstrap/bootout them via `launchctl`, check running state, and read existing agent configs with a minimal API.

## Installation

Add `lunchctl` to your `Cargo.toml` from Git: `lunchctl = { git = "https://github.com/mishamyrt/lunchctl" }`.

## Quick start

Use `LaunchAgent` to define a job (label, `program_arguments`, `run_at_load`, `keep_alive`), then call `write()`, `bootstrap()`, `is_running()`, `boot_out()`, and `remove()`.

## Read an existing agent

Use `LaunchAgent::exists(label)` and `LaunchAgent::from_file(label)` to load an agent and inspect fields like `program_arguments`.

## Examples

See `examples/basic.rs` for an end-to-end flow; run it with `cargo run --example basic`.

## Requirements

Requires macOS with `launchctl` and writes to `~/Library/LaunchAgents` for the current user.

## License

MIT — see `LICENSE` for details.

