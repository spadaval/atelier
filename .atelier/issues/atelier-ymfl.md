---
created_at: "2026-06-12T04:59:03.699058707+00:00"
id: "atelier-ymfl"
issue_type: "epic"
labels:
- "docs"
- "process"
- "rework"
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-v6nd"
  - kind: "issue"
    id: "atelier-wws5"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "closed"
title: "Align docs tests and Agent Factory process with enforced proof"
updated_at: "2026-06-13T00:27:27.360198689+00:00"
---

## Description

Align documentation, tests, and Agent Factory process with enforced proof so
agents cannot close implementation work based on notes, intent summaries, or
unrelated green tests.

## Outcome

- Agent guidance explains when to create new repair issues versus reopening
  misleading closed work.
- The global Agent Factory skill procedures are updated so planning,
  orchestration, implementation, validation, review, docs, and closeout all
  require Outcome/Evidence discipline and attached proof.
- Agent Factory guidance teaches a simple proof rule: ordinary work closes with
  proof on the issue, while risky, broad, or parent-level claims require an
  independent check.
- Planning and closeout guidance require Outcome and Evidence sections on new
  work items once the section parser lands.
- Tests that preserve obsolete behavior are removed, rewritten, or explicitly
  tied to a migration window.
- Ignored tests have named follow-up owners or are deleted if the behavior is no
  longer part of the product.
- Mission closeout process requires a contract audit that maps mission Outcome
  lines to command output, tests, docs, or attached evidence.
- Documentation no longer contradicts the implemented command surface or
  compatibility policy.
- Agent Factory guidance no longer lists `atelier workflow validate` as a
  normal planning, closeout, or drill-down command.

## Evidence

- Docs updates cover Agent Factory workflow, repository guidance, and CLI
  surface documentation.
- Skill updates cover Agent Factory procedure files, not only repository-local
  instructions.
- Process review demonstrates the proof rule on representative issue, epic, and
  mission work.
- Test updates remove stale assertions that old commands are normal behavior.
- A closeout checklist or validator transcript demonstrates each mission
  outcome has evidence.
- Run focused docs/tests, `atelier export --check`, `atelier lint`, and
  `atelier doctor`.

## Notes

This is process repair, not a substitute for command implementation. It should
block mission closeout if documentation and tests still allow the old failure
mode.
