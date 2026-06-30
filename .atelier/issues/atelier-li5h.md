---
created_at: "2026-06-30T16:09:38.303144858+00:00"
id: "atelier-li5h"
issue_type: "epic"
labels:
- "git"
- "workflow"
review:
  kind: pull_request
  number: 35
  provider: forgejo
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-j8ot"
  - kind: "issue"
    id: "atelier-ms7i"
  - kind: "issue"
    id: "atelier-otxv"
  - kind: "issue"
    id: "atelier-qu06"
  children:
  - kind: "issue"
    id: "atelier-3hfq"
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-sszj"
    type: "advances"
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-30T18:13:05.661617718+00:00"
status: "done"
title: "Epic: Define mission integration branch workflow contract"
updated_at: "2026-06-30T18:13:05.661617718+00:00"
---

## Description

Define the durable product and workflow contract for workflow-configured mission integration branches before implementation.

## Outcome

- Product docs, domain language, and an ADR or equivalent architecture note define `base_branch`, work branch, and branch base clearly.
- The contract states that mission integration branches are opt-in through workflow validators and actions.
- Configurable `branch_templates` are removed from the target contract in favor of `<issue_type>/<issue_id>` branch names.
- Projection freshness remains an internal command-storage concern, not a user-configurable workflow validator.
- Default workflow names are simplified from delivery-suffixed names to domain names such as `mission`, `epic`, `task`, `validation`, and `spike`.
- The docs explain that mission scope comes from direct `advances` links plus descendants, not parent hierarchy.

## Evidence

- File diff in product docs or an ADR names the accepted terms and rejected alternatives.
- `target/debug/atelier check atelier-li5h` passes.
