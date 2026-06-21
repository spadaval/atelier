---
created_at: "2026-06-21T17:14:39.006080433+00:00"
id: "atelier-9n3r"
issue_type: "epic"
labels:
- "evidence"
- "workflow"
review:
  kind: pull_request
  number: 21
  provider: forgejo
priority: "P2"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "atelier-e5ye"
  - kind: "issue"
    id: "atelier-lkz6"
  - kind: "issue"
    id: "atelier-mmhf"
  - kind: "issue"
    id: "atelier-od9a"
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
status: "todo"
title: "Epic: Make evidence requirements workflow-driven"
updated_at: "2026-06-21T17:40:57.629614757+00:00"
---

## Description

Move evidence from an implicit completion requirement into configured workflow validators. Evidence records, transcript capture, and attach/show/list remain available as optional capabilities, but ordinary work and status surfaces should not claim proof is missing unless the relevant workflow transition actually configures a validator that requires it.

## Outcome

- Ordinary task workflows can close without attached evidence when their configured close transition does not include an evidence validator.
- `evidence.attached` remains a built-in validator capability with concise failure output and a simple help hint.
- `status`, `issue status`, `issue show`, and blocked transition output derive evidence/readiness guidance from configured validators instead of bespoke proof-gap checks.
- Issue parsing and lint no longer force Evidence sections or concrete proof wording for ordinary issues unless a configured validator or explicit issue contract requires it.
- Evidence record commands and command transcript capture remain available and tested as optional proof artifacts.

## Evidence

- Focused transition tests prove close behavior with and without `evidence.attached` in workflow.yaml.
- CLI transcript or snapshot tests prove status/readiness surfaces show evidence help only when the configured validator fails.
- Lint/parser tests prove ordinary issues are not forced to carry Evidence prose by default.
- Existing evidence command tests continue to pass for record, capture, attach, show, and list behavior.
