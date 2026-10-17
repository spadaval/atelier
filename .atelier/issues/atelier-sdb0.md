---
created_at: "2026-10-16T23:04:57.737893637+00:00"
id: "atelier-sdb0"
issue_type: "feature"
labels:
- "orientation"
workflow_branch:
  branch_base: master
  work_branch: feature/atelier-sdb0
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-1ci8"
  - kind: "issue"
    id: "atelier-d2xe"
  - kind: "issue"
    id: "atelier-jjvc"
  - kind: "issue"
    id: "atelier-mi3x"
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-1w8c"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T02:54:24.193746171+00:00"
status: "done"
title: "Make status the session-start view and retire prime"
updated_at: "2026-10-17T02:54:24.193746171+00:00"
---

## Goal

Per ADR 0022, one command orients a session: `atelier status`. It absorbs what only `prime` shows today for an owner (the owner's body, mission and strategy, recent reviews, and evidence with freshness), adds the top ready items to its ready count, drops `prime`'s static lines ("Guidance: …" and "Before handing off: …"; the Stop hook runs `land`), and becomes the SessionStart hook. Agents stop choosing between two overlapping views.

## Done when

- `atelier status --hook claude` emits SessionStart `additionalContext`, and `atelier setup claude` installs it in place of `prime --hook claude`.
- On an owner branch `status` shows the owner body, mission and strategy path, the 8 newest reviews, and evidence with freshness; without an owner it lists up to 5 ready items under the ready count.
- `prime` prints the same output with a one-line deprecation note.
- The `prime` tests in `crates/atelier-cli/tests/session.rs`, `cli_integration/issues.rs`, `cli_integration/mission_worktree.rs` and `cli_integration/setup_guidance.rs` are moved to `status` or kept as alias tests, and pass.
- No current doc or help text presents `prime` as the session command: `PRODUCT_INTENT.md`, `AGENTS.md`, `docs/product/{sessions-and-hooks,cli-surface,work-model,index}.md`, the root help in `main.rs`, the `setup.rs` success message, and `scripts/bench_read_commands.py`. This repository's `.claude/settings.json` is regenerated with `atelier setup claude` so it calls `status`.
- `sessions-and-hooks.md` states that the session relies on the harness loading AGENTS.md (Claude Code reads it only without a `CLAUDE.md`, or through an `@AGENTS.md` import), and `atelier setup claude` prints a warning, with the fix, when the repository has a `CLAUDE.md` that does not import `@AGENTS.md`. A test covers the warning.
- Agent Factory `references/atelier.md` and `procedures/install.md` say `status`, changed upstream per ADR 0020 and resynced.

## Scope

Session and status commands, `setup claude`, their tests, and the docs named above. `land` and its Stop hook do not change. Historical audits under `docs/architecture/quality/` and the rebuild strategy's S6 text stay as written.

## Context

`prime_text` in `crates/atelier-cli/src/commands/session.rs`; `crates/atelier-cli/src/commands/setup.rs`. Adopters' installed hooks call `prime --hook claude`, so `prime` remains as an alias; its removal is a separate issue.

## Close Reason

status is the session-start view and SessionStart hook; prime is a deprecated alias. Independently reviewed (three rounds) and approved; suites pass after merging master.
