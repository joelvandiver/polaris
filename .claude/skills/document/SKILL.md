---
name: document
description: Checklist for documenting a completed change — what/why, behavior changes, test evidence, CI verification, ops impact, decisions, and follow-ups. Invoke after finishing any change, before considering it done.
---

# Document Change

Walk the user through documenting the change they just made. Ask about or infer each item, then produce the documentation text (commit message body, PR description, or `docs/` entry) for the user to apply. This repo is advisory-only ([[advisory-mode]]) — draft the content; the user commits it.

## Checklist — cover each item

1. **What & why** — one paragraph: the intent and the task/issue it addresses.
2. **Behavior changes** — user-facing or API contract changes; call out breaking changes explicitly.
3. **Test evidence** — unit and integration tests added or updated, and the command to run them.
4. **CI verification** — which pipeline checks prove this change works. If none do, flag it: the change is not verifiable and needs a CI step first (see `/tdd-plan`).
5. **Config/ops impact** — env vars, migrations, deploy steps, rollback notes. Write "none" explicitly rather than omitting.
6. **Decisions & trade-offs** — alternatives considered and why this approach won. Recommend an ADR (`docs/adr/`) when the decision is architectural.
7. **Follow-ups** — known limitations or deferred work; turn each into a task list item so it isn't lost.

## Sizing guidance

- **Small change** (typo, small fix): items 1–4, kept to a few lines in the commit/PR message.
- **Feature or behavior change**: all 7 items in the PR description; update affected `README`/`docs/` pages.
- **Architectural change**: all 7 items plus an ADR.

## Output

Deliver ready-to-paste text: a commit message body or PR description, plus any `docs/` or ADR drafts. Remind the user which existing docs (README, API docs, runbooks) the change makes stale.
