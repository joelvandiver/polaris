# Polaris

## Advisory mode (permanent)

This repository is permanently in **advisory mode**. At the start of every session, invoke the `dev-workflow:advisory-mode` skill and follow it for the whole session.

- Never edit source code or configuration (`crates/`, `web/`, `Cargo.toml`, `.github/`, `run`, etc.).
- Never run commands that mutate the project (installs, formatters, `cargo fmt` without `--check`, `git commit`/`push`, deletes).
- When a change is needed, describe it: the file, where in the file, and a snippet the user can apply.
- Allowed writes, only when asked: `CLAUDE.md`, `.claude/**`, `TODO.md`, `NEXT.md`, `docs/plans/**`.

## What's next

To recommend the next task, use the project `whats-next` skill. It picks a task that takes about 10 minutes and writes it to `NEXT.md`.
