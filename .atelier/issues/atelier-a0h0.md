---
created_at: "2026-06-20T15:55:48.533628698+00:00"
id: "atelier-a0h0"
issue_type: "bug"
labels:
- "cutting-pass"
- "fields"
- "schema"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-xz11"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "review"
title: "Clarify or remove generic issue fields storage"
updated_at: "2026-10-01T21:19:17.840874362+00:00"
---

## Goal

Replace the generic issue fields map with typed strategy and workflow branch
metadata, and align issue, epic, mission, and evidence records with the verified
repo-native tracker contract in ADR 0019.

## Done when

Schema version 2 round-trips typed bodies and rejects the old metadata/version.
CLI regression tests show explicit evidence results, full commit pins, stale
acceptance/code/policy evidence including edit/revert history, newer failure
blocking, and concurrent updates without lost data. Formatting, tracker lint,
workspace tests, and the dead/unused-code scan pass after the change.

## Scope

Core record types, Markdown codecs, app evidence freshness, CLI callers and
locking, public bundle fixtures, and the read-command benchmark generator.
Review/merge gates and session commands remain separate rebuild slices.

## Context

- Strategy: `docs/strategies/atelier-rebuild.md`, step 6 (S3, S7).
- Decision: `docs/adr/0019-verified-repo-native-tracker.md`.
- Shapes: `.agents/skills/agent-factory/references/tracker-records.md`.
- Proof: `crates/atelier-cli/tests/cli_integration/verified_records.rs` and
  schema/shape tests in `crates/atelier-records/src/lib.rs`.

## Notes

- 2026-10-01 [handoff] Atomic schema migration implemented on the mission branch.
  Author proof complete; independent review is pending with the orchestrator.
  The shared-lock regression reproduced a lost label before the lock fix.
  Bundle preview's evidence count and read-time commit validation regressions
  were observed failing and then corrected. Logs: `/tmp/step6-*.log`.

- 2026-10-01 [handoff] Evidence freshness repairs passed independent re-review.
  The exclusive and atomic prune repairs passed independent review in the gates
  candidate. Final integrated proof and completion remain outstanding under
  epic `atelier-1ioo`; the earlier author evidence remains historical.
