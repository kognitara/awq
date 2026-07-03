# AGENTS

This repository is a single Rust CLI project named `lys`. It implements a custom local VCS using a `.lys` directory and SQLite store, plus shell integration, a web UI, email, chat, and project generation features.

## How to build and verify

- Build: `cargo build`
- Run tests: `cargo test`
- Format check: `cargo fmt --all --check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`

## Key entry points

- `Cargo.toml` — package metadata, edition, dependencies, and repository details
- `src/main.rs` — CLI entrypoint and command/subcommand tree
- `src/vcs.rs` — core repository operations, diffs, clone/import, shell/mount, and `.lys` handling
- `src/db.rs` — SQLite schema and `.lys/db/store.db` connection management
- `src/lysrc.rs` — project metadata parsing and configuration
- `src/web.rs` — HTTP/web UI rendering and page generation
- `src/shell.rs` — shell wrapper/integration and `LYS_WEB_TERMINAL` support
- `src/commit.rs`, `src/branch.rs`, `src/tags.rs`, `src/chat.rs`, `src/email.rs` — command-specific behavior

## Project conventions

- Keep CLI command names and arguments stable: the repo exposes commands such as `init`, `new`, `web`, `email`, `branch`, `chat`, `commit`, `tag`, `mount`, `import`, and `clone`
- Modules map closely to subcommands, so prefer local module changes for command behavior
- State is stored in `.lys`; do not make assumptions about a Git repository layout
- Unix-specific code is gated behind `cfg(unix)` because of `nix` dependencies
- Documentation is minimal in the repo; use `README.md` for general repo intent and shell completion hints

## Guidance for AI agents

- Focus on small, targeted changes that preserve CLI behavior and state management semantics
- When fixing issues in command parsing or help text, validate against `src/main.rs` and related command module logic
- Prefer referencing `README.md` or `Cargo.toml` rather than duplicating repo-level documentation
- Avoid changing generated or build artifacts in `target/`
