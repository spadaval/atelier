---
created_at: "2026-06-13T03:00:48.118729570+00:00"
id: "atelier-cbru"
issue_type: "task"
labels:
- "assignee:root"
- "authoring"
- "process"
- "ux"
- "validation"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "atelier-ovs0"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 1
closed_at: "2026-06-13T04:11:51.171068876+00:00"
status: "done"
title: "Define validation placement and anti-red-tape guidance"
updated_at: "2026-06-13T04:11:51.171068876+00:00"
---

## Description

Clarify where validation requirements belong across missions, epics, executable issues, and dedicated validation issues. The goal is enough durable context to validate work without private chat history, while avoiding overlapping validation layers that force agents to specify every detail everywhere.
- Guidance defines what belongs in mission Validation, epic Outcome/Evidence, executable issue Outcome/Evidence, and dedicated validation issue Outcome/Evidence.
- The hierarchy is explicitly non-duplicative: each layer owns a different question rather than repeating lower-level detail.
- Mission validation stays focused on mission-level target state and closeout confidence, not child implementation details.
- Epic outcomes define cohesive product/process results and delegate proof to children or a closeout item.
- Executable issues define the local observable result and local proof.
- Dedicated validation issues define independent review scenarios, classification expectations, and evidence capture, without becoming a second implementation spec.
- Guidance includes an anti-red-tape rule: do not add detail to a higher layer unless it changes scope, risk, sequencing, or parent-level confidence.
- Broad persistence, canonical write, projection refresh, runtime-cache, and
  worktree changes require early concurrency or scenario validation before final
  closeout, not only an end-of-mission audit.
- File-change review of work-item authoring and validation guidance shows the placement model and anti-red-tape rule.
- Review artifact includes before/after examples for a subjective `mission list` information-hierarchy task and a quantitative performance task.
- Review artifact includes an example canonical write or projection-refresh
  issue and shows which command transcript, test, or evidence record provides
  early concurrency proof.
- `atelier lint`, `atelier export --check`, and docs whitespace check commands pass.

## Outcome

Outcome was not specified.

## Evidence

Evidence was not specified.
