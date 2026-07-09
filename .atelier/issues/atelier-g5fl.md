---
created_at: "2026-07-06T17:20:25.694804491+00:00"
id: "atelier-g5fl"
issue_type: "validation"
labels:
- "cli"
- "independent-validation"
- "inventory"
- "mission-dashboard"
review:
  kind: pull_request
  number: 65
  provider: forgejo
fields:
  workflow_branch:
    branch_base: mission/atelier-c0mp
    integration_target: mission/atelier-c0mp
    merge_strategy: squash
    owner_issue_id: atelier-g5fl
    owner_kind: issue
    review_target: mission/atelier-c0mp
    work_branch: validation/atelier-g5fl
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "review"
title: "Validate issue inventory and Mission Overview behavior"
updated_at: "2026-07-09T16:42:13.276236820+00:00"
---

## Description

Independently validate the delivered command split from the mission Outcome and public contracts. Exercise representative mission, epic, child, direct-linked, unassigned, blocked, done, and empty tracker states; compare interactive color with `NO_COLOR` and captured noninteractive output; and inspect help and durable guidance. Treat any prior closed implementation claim as context, not proof of current behavior.

## Outcome

- The validator records pass, fail, blocked, or deferred for each mission Outcome claim and cites the command transcript, test result, diff location, or evidence record used for judgment.
- Validation proves that `issue list` is flat and inventory-shaped; `work missions` hides done missions by default, groups linked epics without default leaf rows, accounts for exceptional work, and preserves meaning with and without color; help, docs, and tracker health agree with the implementation.

## Evidence

- Evidence record `atelier-ppld`: PASS — isolated temporary-repository command transcripts prove flat inventory membership and ordering, metadata filters, quiet/limit/error/empty forms, collapsed mission/epic/child/direct/blocked/unassigned/done behavior, exceptional-work accounting, explicit drilldowns, and default-done versus `--all` semantics.
- Evidence record `atelier-s2em`: PASS — public help, durable product guidance, mission wording, captured output, `NO_COLOR`, and interactive-terminal color agree; color changes styling without removing textual meaning.
- Evidence record `atelier-za8n`: PASS — the `cargo nextest run` test transcript completed 684 tests with 684 passed and 0 skipped. The extended ignored-only profile reported zero ignored tests, so no stale ignored scenario substitutes for validation.
- Quality gates: PASS — `cargo fmt -- --check`, `git diff --check`, `target/debug/atelier check atelier-c0mp`, `target/debug/atelier check atelier-g5fl`, and repository-wide `target/debug/atelier check` pass after rebuilding only disposable cache state with the validation-commit binary.
