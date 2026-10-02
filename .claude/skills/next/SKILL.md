---
name: next
description: Recommend the single next task in Polaris, scoped to about 10 minutes of work, and print it in chat. Use when the user asks "what's next", starts a session, or finishes a task.
---

# What's next (10-minute tasks)

Advisory mode applies (see CLAUDE.md). You recommend and spec the task; the user does it.

## Steps

1. **Assess state** (read-only): `TODO.md`, `docs/plans/`, `git log -10 --oneline`, `git status`, `git diff`. If the code doesn't match the task list, say so in one line.
2. **Check health**: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`. A red check is the next task.
3. **Pick ONE task the user can finish in about 10 minutes.** Priority order:
   1. A broken build, CI run or test
   2. Half-done work in the tree
   3. The next unchecked TODO or roadmap item
   
   If the item is bigger than 10 minutes, take its first slice (for example, one function and its tests) and give the rest as follow-ups.
4. **Spec it TDD-style**, short enough to read in a minute:
   - The failing test(s) to write first: names and assertions as a snippet
   - The minimal implementation: file, location and snippet
   - The command that proves it's done (`cargo test -p polaris-math`, etc.)
5. **Print the full spec in chat.** Don't write or touch `NEXT.md`.

## Chat format

```
# Next: <task> (~10 min)

**Why:** one sentence.

## Steps
1. Write the failing test ... (snippet)
2. Implement ... (snippet)
3. Verify: `<command>`

## Done when
- [ ] <checkable outcome>

## On deck
- <next 10-minute task>
- <next 10-minute task>
```

## Reply in chat

The spec above is the whole reply. Don't survey the options.
