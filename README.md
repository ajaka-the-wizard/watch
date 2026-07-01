# watch

`watch` is a lightweight Rust file watcher and command runner. It monitors a configured directory for file changes and restarts your command whenever matching files are modified.

## Highlights

- Supports `watch.toml` and `watch.json` config files
- Watches directories recursively
- Filters events by file extension
- Restarts commands automatically on file changes
- Graceful shutdown with Ctrl-C handling
- Example config initialization via `init`
- Ready for use and actively evolving

## Project layout

- `Cargo.toml` - Rust package and dependency manifest
- `src/main.rs` - program entry point
- `src/lib.rs` - top-level app orchestration
- `src/configs/cli.rs` - CLI parsing and config initialization
- `src/configs/config.rs` - config file loading and validation
- `src/inner/engine.rs` - file watching and event dispatch
- `src/inner/executor.rs` - command execution and process-group management
- `defaults/` - example `watch.toml` and `watch.json`

## Config

Create either `watch.toml` or `watch.json` in the project root.

Supported fields:

- `command` - shell command to run when files change
- `watch` - list of file extensions to watch
- `dir` - directory to watch
- `verbose` - enable verbose logging

Example `watch.toml`:

```toml
command = "git status"
watch = ["py", "ts", "rs"]
dir = "."
verbose = true
```

## Usage

Initialize an example config file:

```sh
cargo run -- init
```

Watch with the configured file:

```sh
cargo run
```

If you prefer JSON:

```sh
cargo run -- init --json
```

## Runtime behavior

- If both `watch.toml` and `watch.json` exist, startup fails and asks for only one config file.
- If neither config exists, startup fails and prompts you to initialize one.
- On file change, the current process is stopped and the command is restarted.
- Ctrl-C shuts down the watcher and cleans up the running child process.

## Development

This project is actively improved and is not yet `v1`. New features and refinements are added frequently.

Build and run locally:

```sh
cargo run
```

For a release build:

```sh
cargo build --release
```
