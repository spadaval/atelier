---
created_at: "2026-10-17T01:10:57.970791265+00:00"
id: "atelier-fb1i"
issue_type: "bug"
labels:
- "check"
- "cli-output"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "atelier-2mky"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Show graph errors in atelier check even when a record fails to parse"
updated_at: "2026-10-17T01:10:57.970791265+00:00"
---

## Goal

With a YAML error in one record (atelier-wtnp, `status: todo: bad`) and an `advances` link to a missing `atelier-nope` in another (atelier-gfi8), `atelier check` reports only the YAML error. The missing reference appears only after the YAML is fixed and check is rerun. The output never says graph checks were skipped. atelier-idae (done) made check report every independent graph error, but only once every record parses. The Agent Factory skill's Atelier reference (`skills/agent-factory/references/atelier.md` in spadaval/.agents, Administration) says check "stops at the first structural error", which matches neither idae's claim nor this behaviour.

## Done when

- That two-fault fixture reports both problems in one run. If graph checks cannot run safely, check instead prints `Graph checks skipped: fix the records above first`.
- The Administration section of `skills/agent-factory/references/atelier.md` states the actual behaviour. That edit is committed in spadaval/.agents, the skill's one copy (ADR 0023).
- A test covers the two-fault fixture.

## Context

Extends atelier-idae.
