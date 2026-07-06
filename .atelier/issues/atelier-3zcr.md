---
created_at: "2026-06-29T18:21:10.127683093+00:00"
id: "atelier-3zcr"
issue_type: "task"
labels:
- "cli"
- "tests"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ubf2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T15:09:41.082935112+00:00"
status: "done"
title: "Add regression coverage for issue list and mission dashboard behavior"
updated_at: "2026-06-30T15:09:41.082935112+00:00"
---

## Description

Add focused integration coverage for the restored `issue list` inventory command, the repaired `work mission` dashboard, and the resolved `work` bucket-view contract. The tests should prove both the positive command behavior and the removal of confusing old dashboard language.

## Outcome

Integration tests prove `atelier issue list` lists all created issues by default, including done/closed records, and that `--status todo`, `--category done`, `--issue-type mission`, `--ready`, `--blocked`, and `--quiet` return expected rows or IDs. Work-view tests prove `atelier work ready` is the small top-level picker, `atelier work blocked` is the blocker-triage view, normal role guidance does not route through `work queue --ready` or `work queue --blocked`, and any surviving `work queue` behavior has one explicit documented job. Mission dashboard tests prove default output no longer contains `Work Mission`, `Health`, `needs proof`, `Proof gaps`, premature `Close readiness`, empty `Blocked Work`, empty `Backlog Work`, or `atelier work queue --all`; ready counts match `work mission <id> --ready`; open blockers show blocker IDs; and a closeout-ready mission shows the closeout section and transition guidance.
