---
created_at: "2026-10-16T23:04:57.737893637+00:00"
id: "atelier-sdb0"
issue_type: "feature"
labels:
- "orientation"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Make status the session-start view and retire prime"
updated_at: "2026-10-16T23:04:57.737893637+00:00"
---

## Goal

Per ADR 0022, one command orients a session: `atelier status`. It absorbs what only `prime` shows today for an owner (the owner's body, mission and strategy, recent reviews, and evidence with freshness), drops the static "Guidance:" line, and becomes the SessionStart hook. Agents stop choosing between two overlapping views.

## Done when

`atelier status --hook claude` emits the SessionStart `additionalContext` that `prime --hook claude` emits today, minus the guidance line; `atelier setup claude` installs it; on an owner branch `status` shows the owner body, mission and strategy path, the 8 newest reviews and evidence with freshness; `prime` prints the same output with a one-line note that it is deprecated. The existing `prime` tests in `crates/atelier-cli/tests/session.rs` pass against `status`, and `docs/product/sessions-and-hooks.md` and the Agent Factory `references/atelier.md` say `status`.

## Scope

`session.rs`, the status command, `setup claude`, their tests, and the docs named above. `land` and its Stop hook do not change.

## Context

`prime_text` in `crates/atelier-cli/src/commands/session.rs`. Adopters' installed hooks call `prime --hook claude`, which is why `prime` remains as an alias until a later removal.
