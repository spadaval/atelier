---
created_at: "2026-11-08T18:09:03.511948849+00:00"
id: "atelier-xe9g"
issue_type: "task"
labels:
- "land"
workflow_branch:
  branch_base: master
  work_branch: atelier-xe9g
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-8v6s"
    type: "implements"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Remove land's unpushed-base-commits warning"
updated_at: "2026-11-08T18:10:33.714942750+00:00"
---

## Goal

Land stops warning about unpushed base commits, so an agent pushes landed work without asking the human (atelier-8v6s).

## Done when

- [x] `foreign_base_commits_note` and its call in `land_gates` (`crates/atelier-cli/src/commands/session.rs`) are gone, as is the test `land_names_unpushed_base_commits_the_owner_did_not_make_before_pushing` in `crates/atelier-cli/tests/session.rs`.
- [x] `docs/product/sessions-and-hooks.md` drops the paragraph "When the local base branch is ahead of its upstream ..." (atelier-mvzi); the next paragraph, on land judging unpushed base commits on the base itself, stays.
- [x] `cargo fmt -- --check` passes, `cargo clippy -p atelier-cli --all-targets` adds no warnings, and `cargo nextest run -p atelier-cli -E 'test(land)'` passes.
- [x] Agent Factory's `references/atelier.md` (spadaval/.agents) says to push without asking once `atelier land` passes, stopping only if land refuses or a commit holds secrets or personal details.

## Context

Decision atelier-8v6s reverses atelier-mvzi's warning.

## Report

All four Done when lines hold. Land no longer prints the unpushed-base-commits warning, and Agent Factory now tells agents to push once land passes.

- **Line 1:** the function, its one call and its test are removed (41 and 34 lines). `owned` and `base_name` are still used later in `land_gates`, and `first_five` has other callers, so nothing is left dead.
- **Line 2:** the paragraph is gone, and `grep -rn "ahead of its upstream\|no owner made\|confirm they are meant"` over the code and docs now finds nothing.
- **Line 3:** `cargo fmt -- --check` passed. `cargo nextest run -p atelier-cli -E 'test(land)'`: 47 passed. `git diff --check` is clean. Clippy prints 21 warnings, all in files or lines this change does not touch (for example `criteria.rs:139`, `status.rs:425`, `tests/session.rs:1853` and `:3051`).
- **Line 4:** spadaval/.agents branch `feature/atelier-xe9g`, commit `b238c15` on main `23a6bed`, extends the push rule in `skills/agent-factory/references/atelier.md`. It merges cleanly with the in-flight `atelier-l58v` branch, which also edits that file.

### Deviations

- **Done when line 3** said clippy passes "with no warnings". The crate already has 21 warnings elsewhere, so the line now says the change adds none.
- **Line 4 was added after start.** It is Sumeet's second request, written in once the change to the skill was settled.

### Decisions

Follows decision atelier-8v6s. The skill guidance goes in `references/atelier.md`, beside the existing push rule, rather than in `SKILL.md`, because it is about `atelier land`.

### Surprises

None.

### Open items

- `CHANGELOG.md` has no Unreleased section; the removal should be noted when the next release's entry is written.
- The skill's eval scenarios (`evals/agent-factory`, including `refused-land-push`) were not rerun for this one-sentence change.
