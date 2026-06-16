---
created_at: "2026-06-16T19:09:06.784365213+00:00"
id: "atelier-6drt"
evidence_type: "validation"
captured_at: "2026-06-16T19:09:05.518846491+00:00"
command: "bash -lc 'rg -n \"closeout|Closeout|--closeout\" docs/product/zen.md docs/product/work-model.md docs/product/cli-surface.md docs/product/validation.md docs/architecture/quality/validation.md SPEC.md CONTEXT.md AGENTFACTORY.md; test $? -eq 1; rg -n \"completion issue|completion work|completion item|completion worker|completion audit|issue type.*completion|completion:|Completion issue|Completion worker|Completion Notes|validation/validation|validation or validation|validation and validation|Parent readiness|parent readiness|mission shell readiness|final readiness judgment|completion readiness|completion-readiness\" docs/product/zen.md docs/product/work-model.md docs/product/cli-surface.md docs/product/validation.md docs/architecture/quality/validation.md SPEC.md CONTEXT.md AGENTFACTORY.md; test $? -eq 1; rg -n \"built-in lifecycle|configurable mission workflow graph|normal issue transitions|terminal done-category\" docs/product/work-model.md docs/product/validation.md docs/product/cli-surface.md SPEC.md CONTEXT.md AGENTFACTORY.md docs/architecture/quality/validation.md; target/debug/atelier lint atelier-ooyw; git diff --check'"
exit_status: "0"
path: null
uri: null
proof_scope: "scoped to the attached target or summary"
agent_identity: null
independence_level: "unspecified"
target:
  kind: "issue"
  id: "atelier-ooyw"
  role: "validates"
follow_up_ids: []
residual_risks: []
output:
  limit_bytes_per_stream: 4096
  stdout:
    bytes: 802
    summary: "docs/product/work-model.md:91:Missions keep the built-in lifecycle `draft`, `ready`, `active`, and `closed`;\ndocs/product/work-model.md:92:Atelier does not add a configurable mission workflow graph. Issues and epics\ndocs/product/work-model.md:93:remain the workflow-owned records: they move through normal issue transitions\ndocs/product/work-model.md:94:until a terminal done-category status is allowed.\ndocs/product/cli-surface.md:559:| Checkout/worktree context is unclear after interrupted cleanup or a missing worktree | `atelier status` then `atelier worktree status` | Recreate or inspect the mission worktree context, then reconcile canonical issue statuses through normal issue transitions or record edits. There is no separate active-pointer repair path in the target workflow. |\nLint passed.\n"
    truncated: false
  stderr:
    bytes: 0
    summary: ""
    truncated: false
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "atelier-ooyw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 1
status: "pass"
title: "Docs define mission completion model without closeout vocabulary"
updated_at: "2026-06-16T19:09:10.823901997+00:00"
---

Docs define mission completion model without closeout vocabulary

Command: bash -lc 'rg -n "closeout|Closeout|--closeout" docs/product/zen.md docs/product/work-model.md docs/product/cli-surface.md docs/product/validation.md docs/architecture/quality/validation.md SPEC.md CONTEXT.md AGENTFACTORY.md; test $? -eq 1; rg -n "completion issue|completion work|completion item|completion worker|completion audit|issue type.*completion|completion:|Completion issue|Completion worker|Completion Notes|validation/validation|validation or validation|validation and validation|Parent readiness|parent readiness|mission shell readiness|final readiness judgment|completion readiness|completion-readiness" docs/product/zen.md docs/product/work-model.md docs/product/cli-surface.md docs/product/validation.md docs/architecture/quality/validation.md SPEC.md CONTEXT.md AGENTFACTORY.md; test $? -eq 1; rg -n "built-in lifecycle|configurable mission workflow graph|normal issue transitions|terminal done-category" docs/product/work-model.md docs/product/validation.md docs/product/cli-surface.md SPEC.md CONTEXT.md AGENTFACTORY.md docs/architecture/quality/validation.md; target/debug/atelier lint atelier-ooyw; git diff --check'
Exit status: 0

Stdout summary:
docs/product/work-model.md:91:Missions keep the built-in lifecycle `draft`, `ready`, `active`, and `closed`;
docs/product/work-model.md:92:Atelier does not add a configurable mission workflow graph. Issues and epics
docs/product/work-model.md:93:remain the workflow-owned records: they move through normal issue transitions
docs/product/work-model.md:94:until a terminal done-category status is allowed.
docs/product/cli-surface.md:559:| Checkout/worktree context is unclear after interrupted cleanup or a missing worktree | `atelier status` then `atelier worktree status` | Recreate or inspect the mission worktree context, then reconcile canonical issue statuses through normal issue transitions or record edits. There is no separate active-pointer repair path in the target workflow. |
Lint passed.

Stderr summary:
(none)

