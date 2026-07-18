---
name: tdd-plan
description: Structure any feature, fix, or refactor as a TDD plan — failing tests first (full unit + integration scope), implementation steps, CI verification, and documentation. Invoke when planning any change to this repo.
---

# TDD Plan

Produce plans and task lists that enforce test-driven development. This repo is advisory-only ([[advisory-mode]]): you describe the tests and changes; the user writes them.

## Plan structure for every change

For each unit of work, order the steps exactly like this:

1. **Red** — specify the failing test(s) to write first:
   - Name each test, what it asserts, and the fixture/setup it needs.
   - **Unit tests**: cover happy path, edge cases, and error cases for each function/module with logic.
   - **Integration tests**: cover each boundary touched — API endpoints, DB access, external services, component wiring.
2. **Green** — describe the minimal implementation that makes the tests pass.
3. **Refactor** — note cleanups to apply once green, protected by the tests.
4. **CI** — name the CI check(s) that verify this change. If no check covers it, add a plan step to create one. Nothing counts as verifiable unless CI proves it.
5. **Document** — end every plan with a documentation step; use `/document` for the checklist.

## Rules

- Never propose implementation before its tests are specified.
- Reject "we'll add tests later" in any plan — tests are the first step, not a follow-up.
- Call out untested existing code the change touches, and suggest characterization tests before modifying it.
- A task list item is only "done" when: tests written → tests pass → CI green → change documented.

## Test scope checklist (apply to every plan)

- [ ] Unit: logic branches, edge cases, error handling
- [ ] Integration: every external boundary the change touches
- [ ] Regression: a test that would have caught the bug (for fixes)
- [ ] CI: all of the above run in the pipeline, and fail the build when broken
