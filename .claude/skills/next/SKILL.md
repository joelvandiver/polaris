---
name: next
description: Recommend the single next task in Polaris, scoped to about 10 minutes of work, and print it in chat. Optionally filter by item type ("math" or "task") as grouped in TODO.md. Use when the user asks "what's next", starts a session, or finishes a task.
argument-hint: "[math|task]"
---

# What's next (10-minute tasks)

Advisory mode applies (see CLAUDE.md). You recommend and spec the task; the user does it.

## Item types

`TODO.md` groups items under `##` headings. The heading is the item's type:

| Heading          | Type   | Argument aliases        | Usual home              |
| ---------------- | ------ | ----------------------- | ----------------------- |
| `## Mathematics` | `math` | `math`, `mathematics`   | `crates/polaris-math`   |
| `## Task`        | `task` | `task`, `tasks`, `eng`  | any crate, `web/`, CI   |

If `TODO.md` gains a new `##` heading, treat it as a new type named after the heading (lowercased).

## Steps

1. **Read the requested type.** If the user gave one (`/next math`, "what's the next math item"), only pick from that section. With no type, pick from any section.
2. **Assess state** (read-only): `TODO.md`, `docs/plans/`, `git log -10 --oneline`, `git status`, `git diff`. If the code doesn't match the task list, say so in one line.
3. **Check health**: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`. A red check is the next task whatever type was asked for; say in one line that it preempts the requested type.
4. **Pick ONE task the user can finish in about 10 minutes.** Priority order:
   1. A broken build, CI run or test
   2. Half-done work in the tree (of the requested type, if any)
   3. The next unchecked item of the requested type, or with no type, the next unchecked item in any section (prefer the type the recent commits were working on)

   If the item is bigger than 10 minutes (for example "Build number theory module"), take its first slice (one function and its tests) and give the rest as follow-ups.
   If the requested section has no unchecked items, say so in one line and offer the next item of another type.
5. **Spec it TDD-style for its type**, short enough to read in a minute:

   **math**
   - The definition or property in one sentence (for example, "gcd(a, b) is the largest d dividing both").
   - Failing tests first: known values, edge cases (0, 1, negatives, equal inputs, overflow bounds), and one identity checked over a small range (for example, `gcd(a, b) * lcm(a, b) == a * b` for `1..=50`).
   - The minimal implementation in `crates/polaris-math`: module, signature, snippet. Name the algorithm (Euclid, sieve, etc.).
   - Verify: `cargo test -p polaris-math`.

   **task**
   - The behavior change in one sentence.
   - Failing tests first, using the crate's test tooling (`ratatui::backend::TestBackend` for `polaris-tui`, `wasm-bindgen-test` for `polaris-wasm`, plain unit tests elsewhere).
   - The minimal implementation: file, location, snippet, plus any `Cargo.toml` dependency line.
   - Verify: the crate's test command, plus a manual check (run the app, open the page) when the item is about UI or "verify".

6. **Print the full spec in chat.** Don't write or touch `NEXT.md`.

## Chat format

```
# Next [<type>]: <task> (~10 min)

**Why:** one sentence.

## Steps
1. Write the failing test ... (snippet)
2. Implement ... (snippet)
3. Verify: `<command>`

## Done when
- [ ] <checkable outcome>

## On deck
- [<type>] <next 10-minute task>
- [<type>] <next 10-minute task>
```

On deck stays within the requested type. With no type, list the next item of each type.

## Reply in chat

The spec above is the whole reply. Don't survey the options.
