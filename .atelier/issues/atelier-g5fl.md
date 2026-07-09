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
- Evidence records `atelier-s2em`, `atelier-d4fw`, and `atelier-x2m5`: PASS — public help, durable product guidance, and mission wording agree. For both `work missions` and `issue list`, ANSI-stripped interactive text compares byte-for-byte equal to `NO_COLOR` TTY and captured noninteractive text; interactive output contains ANSI styling while the other modes contain no escapes.
- Evidence record `atelier-za8n`: PASS — the captured `cargo nextest run` transcript observably starts 684 tests across nine binaries and records exit status 0. Its bounded output is truncated before nextest's final count summary, so this record makes no exact pass/skip-count claim.
- Evidence record `atelier-k9f4`: PASS — the captured ignored-only command returned nextest's expected no-tests status 4 and explicitly reported 0 tests run and 684 skipped, proving that no ignored validation scenario exists.
- Quality gates: PASS — `cargo fmt -- --check`, `git diff --check`, `target/debug/atelier check atelier-c0mp`, `target/debug/atelier check atelier-g5fl`, and repository-wide `target/debug/atelier check` pass after rebuilding only disposable cache state with the validation-commit binary.
