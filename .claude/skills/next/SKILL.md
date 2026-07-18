---
name: next
description: Determine what to work on next — read TODO.md and the roadmap, check recent commits for progress, and recommend the single next task broken down TDD-style (failing tests first). Invoke at the start of a work session or after finishing a task.
---

# Next

Recommend the user's next unit of work. This repo is advisory-only ([[advisory-mode]]) — you recommend and spec the work; the user does it.

## Steps

1. **Assess state** (read-only):
   - Read `TODO.md` and the current plan under `docs/plans/`.
   - Check `git log` since the last session and `git status` for in-flight work.
   - If reality has drifted from `TODO.md` (done items unchecked, work not in the list), report the drift and offer to update the task list.
2. **Pick ONE task** — the highest-leverage unchecked item in the current milestone. Prefer, in order:
   - Finishing anything half-done (uncommitted or failing-CI work) over starting new work.
   - Unblocking tasks (things other tasks depend on).
   - The next unchecked item in milestone order.
3. **Spec it TDD-style** (per `/tdd-plan`):
   - The exact failing test(s) to write first — names, assertions, fixtures.
   - The minimal implementation to make them pass.
   - The CI job(s) that will verify it.
   - The documentation step (`/document`).
4. **Flag blockers** — missing decisions, unclear requirements, or dependencies. If a decision is the user's to make, present the options with a recommendation.

## Output format

- **Now**: the one recommended task, with its TDD breakdown.
- **Why this**: one or two sentences.
- **On deck**: the following 2–3 tasks, one line each.
- **Blockers/drift**: only if any exist.

Do not start doing the task. End by asking if the user wants the task specced in more detail or wants to update `TODO.md`.
