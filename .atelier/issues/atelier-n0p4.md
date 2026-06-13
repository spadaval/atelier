---
created_at: "2026-06-13T17:29:11.073952060+00:00"
id: "atelier-n0p4"
issue_type: "feature"
labels:
- "workflow"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-9t3z"
  - kind: "issue"
    id: "atelier-fyms"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T19:20:09.496495076+00:00"
status: "done"
title: "Implement workflow init and status migration"
updated_at: "2026-06-13T19:20:09.496495076+00:00"
---

## Description

Add the bootstrap and migration commands needed for existing repositories to adopt repo-defined issue workflows deliberately. This work creates the starter policy and migrates old hardcoded issue statuses without compatibility readers.
- atelier workflow init writes .atelier/workflow.yaml with standard and lightweight starter workflows and refuses to overwrite an existing policy unless --force is supplied.
- atelier workflow migrate-statuses maps open to todo, closed to done, and archived to archived in canonical issue Markdown.
- Migration preserves close timing and close-reason activity where present and refuses missing or invalid workflow policy.
- Mutating workflow commands can point unmigrated repositories to the migration command instead of silently accepting old statuses.
- Fresh-repo and existing-repo CLI tests cover init refusal, force behavior, migration success, and invalid-policy migration failure.
- Canonical Markdown diff review shows status migration results for open, closed, and archived fixtures.
- atelier workflow check, atelier lint, and atelier export --check pass after migration scenarios.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
